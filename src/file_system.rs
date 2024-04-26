// TODO: Allow the loading to be called directly with a key instead of automatically opening a file dialog every time.
// TODO: Record the current filepath for regular saving, only Save As works currently.
// TODO: The app crashes when the user closes a file dialog or tries to select a wrong file. Fix this.

use bevy::ecs::query::*;
use bevy::ecs::system::SystemState;
use bevy::prelude::*;
use bevy_save::*;
use std;
use std::path::PathBuf;

use crate::error::Error;
use crate::juice_renderer::link_particle_sprite;
use crate::simulation::{
    SimConstraints, SimDrain, SimFaucet, SimGrid, SimGridCellType, SimParticle, SimSurfaceDirection,
};

use std::io::{Read, Write};

use serde::{de::DeserializeSeed, Serialize};

pub struct FileSystem;
impl Plugin for FileSystem {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentFile::default());

        // Setting up the type registry so the data can be accessed

        // Registering SimParticle and it's associated types
        app.register_type::<SimParticle>();
        app.register_type::<Option<Vec2>>(); // Needed for loading position, velocity, and any other Vec2 types

        // Registering SimConstraints
        // All associated types are f32, usize, u8, and Vec2. All already registered
        app.register_type::<SimConstraints>();
		app.register_type::<(Entity, Vec2)>();
		app.register_type::<Vec<(Entity, Vec2)>>();

        // Registering SimGrid and it's associated types
        app.register_type::<SimGrid>();
        app.register_type::<(u16, u16)>(); // Needed for loading dimensions
        app.register_type::<SimGridCellType>();
        app.register_type::<Vec<SimGridCellType>>();
        app.register_type::<Vec<Vec<SimGridCellType>>>(); // Needed for loading the cell_type
        app.register_type::<Vec<f32>>();
        app.register_type::<Vec<Vec<f32>>>(); // Needed for loading cell_center, velocity_u, velocity_v, and density
        app.register_type::<Vec<Entity>>();
        app.register_type::<Vec<Vec<Entity>>>(); // Needed for loading spatial_lookup
        app.register_type::<Option<Rect>>(); // Pretty sure needed for loading any <Vec<Vec<T>>>()

        // Registering SimFaucet, SimDrain, and their associated types
        app.register_type::<SimFaucet>();
        app.register_type::<SimDrain>();
        app.register_type::<SimSurfaceDirection>();

        // Loading and saving funcitonality is called using Bevy's state transitions
        // Since they have direct world and file access, they freeze all other processes. This is to prevent them being scheduled in Update.
        app.add_state::<JuiceStates>();
        app.add_systems(OnEnter(JuiceStates::Loading), load_scene);
        app.add_systems(OnEnter(JuiceStates::Saving), save_scene);
        app.add_systems(OnExit(JuiceStates::Running), reset_state); // Scheduled after load_scene or save_scene since it can't run in parellel.
    }
}

#[derive(Resource)]
pub struct CurrentFile {
    filepath: String,
}

impl Default for CurrentFile {
    fn default() -> CurrentFile {
        Self {
            filepath: String::from("assets/scenes/my file"),
        }
    }
}

impl CurrentFile {
    fn new(filepath: String) -> Self {
        Self { filepath: filepath }
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum JuiceStates {
    Running,
    Saving,
    Loading,
}

impl Default for JuiceStates {
    fn default() -> JuiceStates {
        JuiceStates::Running
    }
}

/// Custom file format. Extension is set to .juice, but under the hood it's really just json.
/// Connects to bevy_save's JSONFormat implementation and uses that.
pub struct JUICEFormat;

impl Format for JUICEFormat {
    /// Returns name of file extension added to file.
    fn extension() -> &'static str {
        ".juice"
    }

    /// Setting bevy_save's JSON serializer to JUICEFormat's serializer. This creates the file from the resources/entities.
    fn serialize<W: Write, T: Serialize>(writer: W, value: &T) -> Result<(), bevy_save::Error> {
        JSONFormat::serialize(writer, value)
    }

    // Setting bevy_save's JSON deserializer to JUICEFormat's deserializer. This creates the resources/entities from the file.
    fn deserialize<R: Read, S: for<'de> DeserializeSeed<'de, Value = T>, T>(
        reader: R,
        seed: S,
    ) -> Result<T, bevy_save::Error> {
        JSONFormat::deserialize(reader, seed)
    }
}

/// Pipeline for saving and loading files. Contains current key (filepath) and an implementation of bevy_save's Pipeline
struct JuicePipeline {
    key: String, // The full filepath for the location of the file.
}

impl JuicePipeline {
    pub fn new(key: String) -> Self {
        Self { key: key }
    }
}

impl Pipeline for JuicePipeline {
    type Backend = DefaultDebugBackend; // Interface between file system, custome file format, and our Rust code, bevy_save handles this.
    type Format = JUICEFormat; // Connecting to the .juice custom file format, really JSON.

    type Key<'a> = &'a str;

    fn key(&self) -> Self::Key<'_> {
        return &self.key;
    }

    /// Generates a snapshot of bevy's world, the current SimGrid, SimConstraints, all SimParticles,
    /// all SimDrains, and all SimFaucets.
    ///
    /// This is the Pipeline's way to save files. Most of the implementation is in bevy_save.
    fn capture(builder: SnapshotBuilder) -> Snapshot {
        builder
			.deny_all()
			.allow::<SimGrid>()
			.allow::<SimConstraints>()
			.allow::<SimParticle>()
			// .allow::<SimFaucet>()
			// .allow::<SimDrain>()
            .extract_resource::<SimGrid>()
            .extract_resource::<SimConstraints>()
            .extract_entities_matching(|e| e.contains::<SimParticle>())
			// .extract_entities_matching(|e| e.contains::<SimFaucet>())
			// .extract_entities_matching(|e| e.contains::<SimDrain>())
            .build()
    }

    /// Despawns all SimParticles, SimDrains, and SimFaucets in the current world,
    /// then loads a snapshot generated from a file.
    ///
    /// This is the Pipeline's way to load files. Most of the implementation is in bevy_save.
    fn apply(world: &mut World, snapshot: &Snapshot) -> Result<(), bevy_save::Error> {
        snapshot
            .applier(world)
            .despawn::<Or<(With<SimParticle>, With<SimFaucet>, With<SimDrain>)>>() // Despawning all entities.
            .apply()
    }
}

/// Sets file_system.rs state to Saving, which triggers save_scene() to run.
/// If ask_user_for_file is true, run save-as and ask user for file.
pub fn init_saving(
    ask_user_for_file: bool,
    current_file: &mut CurrentFile,
    mut file_state: ResMut<NextState<JuiceStates>>,
) {
    // Run Save-As functionality instead of Save functionality
    if ask_user_for_file {
        let key = create_new_file();

        match key {
            Ok(key) => {
                // TODO: validate the key as a real filepath
                current_file.filepath = key;
            }
            Err(e) => { // No file selected, cancel saving.
                println!("{}", Error::FileExplorer("User did not select file."));
                return ()
            },
        }
    }

    file_state.set(JuiceStates::Saving); // Triggers save_scene()
}

/// Triggers a file dialog asking user for filepath, saves the data into the file. Function runs when state = JuiceStates::Saving.
/// Does nothing if user doesn't select a file.
fn save_scene(world: &mut World) {
    let key: String = match world.get_resource::<CurrentFile>() {
        Some(current_file) => current_file.filepath.clone(),
        None => return (), /*world.get_resource::<CurrentFile>().unwrap().filepath.clone()*/ // TODO run save as here
    };

    world
        .save(JuicePipeline::new(key))
        .expect("Did not save correctly, perhaps filepath was incorrect?");
}

/// Sets file_system.rs state to Loading, which triggers load_scene() to run.
/// UNFINISHED FUNCTIONALITY - If a String is passed in the key argument, load that function. Otherwise run a file dialog asking the user.
pub fn init_loading(
    key: Option<String>,
    current_file: &mut CurrentFile,
    mut file_state: ResMut<NextState<JuiceStates>>,
) {
    // If key was set as a parameter, load from there. Otherwise call Pick File file dialog, get_file().
    match key {
        Some(key) => {
            // TODO: Test to see if key is valid
            current_file.filepath = key;
        }
        None => {
            let key = get_file();
            match key {
                Ok(key) => {
                    current_file.filepath = key;
                }
                Err(e) => { // No file selected, cancel loading.
                    println!("{}", Error::FileExplorer("User did not select file."));
                    return ()
                },
            }
            // TODO: Set key as string returned
        }
    }

    file_state.set(JuiceStates::Loading); // Triggers load_scene()
}

/// Runs file dialog asking user for filepath, loads the file into the world. Function runs when state = JuiceStates::Loading.
fn load_scene(world: &mut World) {
    let key: String = match world.get_resource::<CurrentFile>() {
        Some(current_file) => current_file.filepath.clone(),
        None => return (), /*world.get_resource::<CurrentFile>().unwrap().filepath.clone()*/
    };

    world
        .load(JuicePipeline::new(key))
        .expect("Did not load correctly, perhaps filepath was incorrect?");

	// Erase the spatial lookup table, this will cause "ghost particles" otherwise.
	if let Some(mut grid) = world.get_resource_mut::<SimGrid>() {
		grid.spatial_lookup = vec![vec![Entity::PLACEHOLDER; 0]; grid.dimensions.0 as usize * grid.dimensions.1 as usize];
	}

	// Pause the simulation once we have loaded in!
	if let Some(mut constraints) = world.get_resource_mut::<SimConstraints>() {
		constraints.is_paused = true;
	} else {
		println!("Constraints not constructed in time; cannot pause!");
	}
}

/// Sets state back to JuiceStates::Running.
fn reset_state(mut file_state: ResMut<NextState<JuiceStates>>) {
    file_state.set(JuiceStates::Running);
}

/// Triggers a file dialog asking user to select an existing .juice file. Returns the path to it as an Option<String>.
fn get_file() -> Result<String, Error> {
    let start_path = match std::env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            return Err(Error::FileExplorer(
                "Invalid starting directory or could not connect to file explorer",
            ))
        }
    };

    let selected_path: PathBuf = match rfd::FileDialog::new()
        .add_filter("text", &["juice"])
        .set_directory(&start_path)
        .pick_file()
    {
        Some(path) => path,
        None => return Err(Error::FileExplorer("Invalid file selection")),
    };

    let full_key: String = match selected_path.into_os_string().into_string() {
        Ok(path) => path,
        Err(e) => {
            return Err(Error::FileExplorer(
                "Format of path is invalid, cannot convert to String",
            ))
        }
    };

    let key: &mut String = &mut full_key.clone(); // Adding mutability to allow for truncation.

    key.truncate(key.len() - 6); // Removing the .juice file extension, bevy_save breaks otherwise.

    Ok(key.to_string()) // Removing mutability
}

/// Runs a file dialog asking user to create a new .juice file. Returns the path to it as an Option<String>.
///
/// Does not actually create a file, just passes a String to where one should be created.
fn create_new_file() -> Result<String, Error> {
    let start_path = match std::env::current_dir() {
        Ok(path) => path,
        Err(e) => {
            return Err(Error::FileExplorer(
                "Invalid starting directory or could not connect to file explorer",
            ))
        }
    };

    let selected_path: PathBuf = match rfd::FileDialog::new()
        .add_filter("text", &["juice"])
        .set_directory(&start_path)
        .save_file()
    {
        Some(path) => path,
        None => return Err(Error::FileExplorer("Invalid file selection")),
    };

    let full_key: String = match selected_path.into_os_string().into_string() {
        Ok(path) => path,
        Err(e) => {
            return Err(Error::FileExplorer(
                "Format of path is invalid, cannot convert to String",
            ))
        }
    };

    let key: &mut String = &mut full_key.clone(); // Adding mutability to allow for truncation.

    key.truncate(key.len() - 6); // Removing the .juice file extension, bevy_save breaks otherwise.

    Ok(key.to_string()) // Removing mutability
}
