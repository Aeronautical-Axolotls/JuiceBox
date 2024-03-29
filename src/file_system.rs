// TODO: Allow the loading to be called directly with a key instead of automatically opening a file dialog every time.
// TODO: The app crashes when the user closes a file dialog or tries to select a wrong file. Fix this.

use bevy::ecs::query::*;
use bevy::prelude::*;
use bevy_save::*;
use std;

use crate::simulation::{SimConstraints, SimGrid, SimGridCellType, SimParticle};

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

pub struct JuicePipeline {
    key: String,
}

impl JuicePipeline {
    pub fn new(key: String) -> Self {
        Self {
            key: key,
        }
    }
}

impl Pipeline for JuicePipeline {
    type Backend = DefaultDebugBackend;
    type Format = DefaultDebugFormat;

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

fn save_scene(world: &mut World) {
    let key = create_new_file();

    world
        .save(JuicePipeline::new(key))
        .expect("Should have saved correctly");
}

fn load_scene(world: &mut World) {
    let key = get_file();

    world
        .load(JuicePipeline::new(key))
        .expect("Should have loaded correctly");
}

fn reset_state(mut file_state: ResMut<NextState<JuiceStates>>) {
    file_state.set(JuiceStates::Running);
}

fn get_file() -> String {
    let start_path = std::env::current_dir().unwrap(); // TODO: set this to assets

    let key: &mut String = &mut rfd::FileDialog::new()
        .add_filter("text", &["json"])  // Only allowing to select .json
        .set_directory(&start_path)                      // Setting the initial folder of the file dialog to the assets folder
        .pick_files()
        .unwrap()[0]
        .clone()                                            // Allows us to move PathBuf since it can't be copied on it's own
        .into_os_string()
        .into_string()
        .expect("Wasn't able to parse filepath");
    
    key.truncate(key.len() - 5); // Removing the .json file extension
    
    key.to_string() // Removing mutability
}

fn create_new_file() -> String {
    let start_path = std::env::current_dir().unwrap(); // TODO: set this to assets

    let key: &mut String = &mut rfd::FileDialog::new()
        .add_filter("text", &["json"])
        .set_directory(&start_path)
        .save_file()
        .unwrap()
        .into_os_string()
        .into_string()
        .expect("Wasn't able to parse filepath");

    key.truncate(key.len() - 5); // Removing the .json file extension
    
    key.to_string() // Removing mutability
}
