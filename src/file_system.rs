// TODO: Allow the loading to be called directly with a key instead of automatically opening a file dialog every time.
// TODO: Record the current filepath for regular saving, only Save As works currently.
// TODO: The app crashes when the user closes a file dialog or tries to select a wrong file. Fix this.

use bevy::ecs::query::*;
use bevy::prelude::*;
use bevy_save::*;
use std;
use std::path::PathBuf;

use crate::error::Error;
use crate::simulation::{
    SimConstraints,
	SimDrain,
	SimFaucet,
	SimGrid,
	SimGridCellType,
	SimParticle,
	SimSurfaceDirection,
};
use crate::ui::UIStateManager;

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
        app.add_systems(OnEnter(JuiceStates::New), handle_new_scene);
        app.add_systems(OnEnter(JuiceStates::Loading), handle_loading);
        app.add_systems(OnEnter(JuiceStates::Reloading), handle_reloading);
        app.add_systems(OnEnter(JuiceStates::Saving), handle_saving);
        app.add_systems(OnEnter(JuiceStates::SavingAs), handle_saving_as);
        app.add_systems(OnExit(JuiceStates::Running), reset_file_state); // Scheduled after handle_loading or handle_saving since it can't run in parellel.
    }
}

#[derive(Resource)]
pub struct CurrentFile {
    filepath: String,
}

impl Default for CurrentFile {
    fn default() -> CurrentFile {
        Self {
            filepath: String::from("saves/my-file"),
        }
    }
}

impl CurrentFile {
    fn _new(filepath: String) -> Self {
        Self { filepath: filepath }
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum JuiceStates {
    Running,
    New,
    Loading,
    Reloading,
    Saving,
    SavingAs,
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

fn handle_new_scene(world: &mut World) {

    // Creates new file dialog asking the user to create new file.
    let key: String = match create_new_file() {
        Ok(filepath) => filepath,
        Err(_e) => {
            println!("{}", Error::FileExplorer("User did not select file."));
            return ()
        },
    };

    // Setting CurrentFile to new file user just created.
    if let Some(mut current_file) = world.get_resource_mut::<CurrentFile>() {
        current_file.filepath = key.clone();
    };

    load_scene(String::from("metadata/default-file"), world);
    save_scene(key, world);
}

/// Runs file dialog asking user for filepath, loads the file into the world. Function runs when state = JuiceStates::Loading.
fn handle_loading(world: &mut World) {
    // Creates new file dialog asking the user to select an existing file.
    let key: String = match get_file() {
        Ok(filepath) => filepath,
        Err(_e) => {
            println!("{}", Error::FileExplorer("User did not select file."));
            return ()
        },
    };

    // Setting CurrentFile to new file user just created.
    if let Some(mut current_file) = world.get_resource_mut::<CurrentFile>() {
        current_file.filepath = key.clone();
    };

    load_scene(key, world);
}

fn handle_reloading(world: &mut World) {
    let key: String = match world.get_resource::<CurrentFile>() {
        Some(current_file) => current_file.filepath.clone(),
        None => return (), /*world.get_resource::<CurrentFile>().unwrap().filepath.clone()*/ // TODO run save as here
    };

    load_scene(key, world);
}

/// Triggers a file dialog asking user for filepath, saves the data into the file. Function runs when state = JuiceStates::Saving.
/// Does nothing if user doesn't select a file.
fn handle_saving(world: &mut World) {
    let key: String = match world.get_resource::<CurrentFile>() {
        Some(current_file) => current_file.filepath.clone(),
        None => return (), /*world.get_resource::<CurrentFile>().unwrap().filepath.clone()*/ // TODO run save as here
    };

    save_scene(key, world);
}

fn handle_saving_as(world: &mut World) {
    // Creates new file dialog asking the user to create new file.
    let key: String = match create_new_file() {
        Ok(filepath) => filepath,
        Err(_e) => {
            println!("{}", Error::FileExplorer("User did not select file."));
            return ()
        },
    };

    // Setting CurrentFile to new file user just created.
    if let Some(mut current_file) = world.get_resource_mut::<CurrentFile>() {
        current_file.filepath = key.clone();
    };

    save_scene(key, world);
}

/// Sets state back to JuiceStates::Running.
fn reset_file_state(mut file_state: ResMut<NextState<JuiceStates>>, mut ui_state_manager: ResMut<UIStateManager>) {
    println!("RESET FILE STATE RUN!");
    file_state.set(JuiceStates::default());
    ui_state_manager.file_state = JuiceStates::default();
}

/// Triggers a file dialog asking user to select an existing .juice file. Returns the path to it as an Option<String>.
fn get_file() -> Result<String, Error> {
    let start_path = match std::env::current_dir() {
        Ok(path) => path,
        Err(_e) => {
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
        Err(_e) => {
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
        Err(_e) => {
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
        Err(_e) => {
            return Err(Error::FileExplorer(
                "Format of path is invalid, cannot convert to String",
            ))
        }
    };

    let key: &mut String = &mut full_key.clone(); // Adding mutability to allow for truncation.

    key.truncate(key.len() - 6); // Removing the .juice file extension, bevy_save breaks otherwise.

    Ok(key.to_string()) // Removing mutability
}

/// Initiate new pipeline and load scene to key.
fn load_scene(key: String, world: &mut World) {
    match world.load(JuicePipeline::new(key)) {
        Ok(_ok) => {

        },
        Err(_e) => {
            println!("{}", Error::FileExplorer("Did not load correctly, perhaps filepath was incorrect or file was corrupted?"));
            return ()
        },
    }

	// Erase the spatial lookup table, this will cause "ghost particles" otherwise.
	if let Some(mut grid) = world.get_resource_mut::<SimGrid>() {
		grid.spatial_lookup = vec![vec![Entity::PLACEHOLDER; 0]; grid.dimensions.0 as usize * grid.dimensions.1 as usize];
	} else {
		println!("Grid not constructed in time; please reset simulation before continuing!");
	}

	// Pause the simulation once we have loaded in!
	if let Some(mut constraints) = world.get_resource_mut::<SimConstraints>() {
		constraints.is_paused = true;
	} else {
		println!("Constraints not constructed in time; cannot pause!");
	}
}

/// Initiate new pipeline and save scene to key.
fn save_scene(key: String, world: &mut World) {
    match world.save(JuicePipeline::new(key)) {
        Ok(_ok) => {

        },
        Err(_e) => {
            println!("{}", Error::FileExplorer("Did not save correctly, perhaps filepath was incorrect?"));
            return ()
        },
    }
}