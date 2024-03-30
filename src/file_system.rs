// TODO: Allow the loading to be called directly with a key instead of automatically opening a file dialog every time.
// TODO: Record the current filepath for regular saving, only Save As works currently.
// TODO: The app crashes when the user closes a file dialog or tries to select a wrong file. Fix this.

use bevy::ecs::query::*;
use bevy::prelude::*;
use bevy_save::*;
use std;

use crate::simulation::{SimConstraints, SimGrid, SimGridCellType, SimParticle};

use std::io::{
    Read,
    Write,
};

use serde::{
    de::DeserializeSeed,
    Serialize,
};

pub struct FileSystem;
impl Plugin for FileSystem {
    fn build(&self, app: &mut App) {
        app.add_state::<JuiceStates>();

        // Setting up the type registry so the data can be accessed
        app.register_type::<SimParticle>();
        app.register_type::<Option<Vec2>>();
        app.register_type::<Option<Rect>>();
        app.register_type::<SimConstraints>();
        app.register_type::<SimGrid>();
        app.register_type::<SimGridCellType>();
        app.register_type::<(u16, u16)>();
        app.register_type::<Vec<Vec<SimGridCellType>>>();
        app.register_type::<Vec<SimGridCellType>>();
        app.register_type::<Vec<Vec<f32>>>();
        app.register_type::<Vec<f32>>();
        app.register_type::<Vec<Vec<Entity>>>();
        app.register_type::<Vec<Entity>>();

        app.add_systems(OnEnter(JuiceStates::Loading), load_scene);
        app.add_systems(OnEnter(JuiceStates::Saving), save_scene);
        app.add_systems(OnExit(JuiceStates::Running), reset_state);
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
    fn extension() -> &'static str {
        ".juice"
    }

    fn serialize<W: Write, T: Serialize>(writer: W, value: &T) -> Result<(), Error> {
        JSONFormat::serialize(writer, value)
    }

    fn deserialize<R: Read, S: for<'de> DeserializeSeed<'de, Value = T>, T>(
        reader: R,
        seed: S,
    ) -> Result<T, Error> {
        JSONFormat::deserialize(reader, seed)
    }
}

struct JuicePipeline {
    key: String,
}

impl JuicePipeline {
    pub fn new(key: String) -> Self {
        Self { key: key }
    }
}

impl Pipeline for JuicePipeline {
    type Backend = DefaultDebugBackend;
    type Format = JUICEFormat;

    type Key<'a> = &'a str;

    fn key(&self) -> Self::Key<'_> {
        return &self.key;
    }

    fn capture(builder: SnapshotBuilder) -> Snapshot {
        builder
            .extract_resource::<SimGrid>()
            .extract_resource::<SimConstraints>()
            .extract_entities_matching(|e| e.contains::<SimParticle>())
            .build()
    }

    fn apply(world: &mut World, snapshot: &Snapshot) -> Result<(), bevy_save::Error> {
        snapshot
            .applier(world)
            .despawn::<With<SimParticle>>()
            .apply()
    }
}

/// Sets file_system.rs state to Saving, which triggers save_scene() to run.
/// UNFINISHED FUNCTIONALITY - If a String is passed in the key argument, save into that function. Otherwise run a file dialog asking the user.
pub fn init_saving(key: Option<String>, mut file_state: ResMut<NextState<JuiceStates>>) {
    if (key.is_some()) {
        println!("Key was provided: {}", key.unwrap());
        // TODO: Test to see if key is valid
        // TODO: Set key as argument
    } else {
        println!("No key was provided");
        // TODO: Run create_new_file() to create file dialog for user
        // TODO: Set key as string returned
    }

    file_state.set(JuiceStates::Saving); // Triggers save_scene()
}

/// Triggers a file dialog asking user for filepath, saves the data into the file. Function runs when state = JuiceStates::Saving.
/// Does nothing if user doesn't select a file.
fn save_scene(world: &mut World) {
    let key = create_new_file(); // TODO: Run this in init_saving() and find a way to pass it into here.

    if (key.is_some()) {
        world
            .save(JuicePipeline::new(key.unwrap()))
            .expect("Did not save correctly, perhaps filepath was incorrect?");
    }
}

/// Sets file_system.rs state to Loading, which triggers load_scene() to run.
/// UNFINISHED FUNCTIONALITY - If a String is passed in the key argument, load that function. Otherwise run a file dialog asking the user.
pub fn init_loading(key: Option<String>, mut file_state: ResMut<NextState<JuiceStates>>) {
    if (key.is_some()) {
        println!("Key was provided: {}", key.unwrap());
        // TODO: Test to see if key is valid
        // TODO: Set key as argument
    } else {
        println!("No key was provided");
        // TODO: Run get_file() to create file dialog for user
        // TODO: Set key as string returned
    }

    file_state.set(JuiceStates::Loading); // Triggers load_scene()
}

/// Runs file dialog asking user for filepath, loads the file into the world. Function runs when state = JuiceStates::Loading.
fn load_scene(world: &mut World) {
    let key = get_file(); // TODO: Run this in init_loading() and find a way to pass it into here.

    if (key.is_some()) {
        world
            .load(JuicePipeline::new(key.unwrap()))
            .expect("Did not save correctly, perhaps filepath was incorrect?");
    }
}

/// Sets state back to JuiceStates::Running.
fn reset_state(mut file_state: ResMut<NextState<JuiceStates>>) {
    file_state.set(JuiceStates::Running);
}

/// Triggers a file dialog asking user to select an existing .juice file. Returns the path to it as an Option<String>.
fn get_file() -> Option<String> {
    let start_path = std::env::current_dir().unwrap();

    let selected_path = rfd::FileDialog::new()
        .add_filter("text", &["juice"]) // Only allowing to select .json
        .set_directory(&start_path) // Setting the initial folder of the file dialog to the assets folder
        .pick_files();

    if (selected_path.is_some()) {
        let key: &mut String = &mut selected_path
            .unwrap()[0]
            .clone() // Allows us to move PathBuf since it can't be copied on it's own
            .into_os_string()
            .into_string()
            .expect("Wasn't able to parse filepath");

        key.truncate(key.len() - 6); // Removing the .juice file extension, bevy_save breaks otherwise.

        return Some(key.to_string()); // Removing mutability
    } else {
        return None;
    }
}

/// Runs a file dialog asking user to create a new .juice file. Returns the path to it as an Option<String>.
///
/// Does not actually create a file, just passes a String to where one should be created.
fn create_new_file() -> Option<String> {
    let start_path = std::env::current_dir().unwrap();

    let selected_path = rfd::FileDialog::new()
        .add_filter("text", &["juice"])
        .set_directory(&start_path)
        .save_file();

    if (selected_path.is_some()) {
        let key: &mut String = &mut selected_path
            .unwrap()
            .into_os_string()
            .into_string()
            .expect("Wasn't able to parse filepath");

        key.truncate(key.len() - 6); // Removing the .juice file extension, bevy_save breaks otherwise.

        return Some(key.to_string()); // Removing mutability
    } else {
        return None;
    }
}