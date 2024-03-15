// https://www.youtube.com/watch?v=bbBh3oKibkE&t=351s&ab_channel=PhaestusFox
// https://crates.io/crates/open
// https://stackoverflow.com/questions/73311644/get-path-to-selected-files-in-active-explorer-window
// https://github.com/Zeenobit/
// https://bevy-cheatbook.github.io/builtins.html#schedules
// https://crates.io/crates/filepath

// EXCLUSIVE SYSTEMS
// Looks like reading / writing to the world blocks all other systems, so they can't run in parellel.
// I need to add an exclusive system since I need that read access for saving. Probably for loading too?
// https://github.com/bevyengine/bevy/issues/4158
// https://www.reddit.com/r/bevy/comments/t6xffl/is_there_a_way_to_get_an_mut_world_in_a_system/
// https://github.com/bevyengine/bevy/blob/main/examples/ecs/ecs_guide.rs GO TO LINE 198!!!!

// CURRENT CHECKLIST TO FIGURE OUT!!!!
// File Saving:
// Am I actually connecting to the correct world?
// How do I receive the data from the world all at once to write it into a file?
// How should I deal with needing the save function to be an exclusive system?
// How do I create a UI button with file saving if I would need to make the whole UI exclusive...?
// What information am I getting when I call println!("{:#?}", world.entities());? The entities are listed, but it doesn't contain useful information.
// Iterate over the entities with world.iter_entities() and try to get specific data individally.
//
// File Loading:
// How can I easily wipe the pre-existing data to add the new data?
// How can I link the particles I add to the world

// Double Buffering:
// https://docs.rs/bevy_double_res/latest/bevy_double_res/all.html
// 


use bevy::utils::petgraph::visit::Time;
use bevy::{prelude::*, tasks::IoTaskPool, utils::Duration};
use std::time::SystemTime;
use std::{fs::File, io::Write};
use bevy::ecs::query::*;
use bevy_save::*;

use crate::simulation::{SimParticle, SimGrid, SimConstraints};
use crate::juice_renderer;

use super::*;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash)]
pub enum JuiceStates {
	Running,
	Saving,
	Loading
}

impl Default for JuiceStates {
	fn default() -> JuiceStates {
		JuiceStates::Running
	}
}

struct JuicePipeline;

impl Pipeline for JuicePipeline {
    type Backend = DefaultDebugBackend;
    type Format = DefaultDebugFormat;

    type Key<'a> = &'a str;

    fn key(&self) -> Self::Key<'_> {
        "assets/scenes/save_test/test_save_3_bevy_save"
        //"assets/scenes/load_test/test_load_11_friends"
    }

    fn capture(builder: SnapshotBuilder) -> Snapshot {
        builder
            .extract_resource::<SimGrid>()
            .extract_resource::<SimConstraints>()
            .extract_entities_matching(|e| e.contains::<SimParticle>())
            //.extract_all_resources()
            .build()
    }

    fn apply(world: &mut World, snapshot: &Snapshot) -> Result<(), bevy_save::Error> {
        snapshot
            .applier(world)
            .despawn::<With<SimParticle>>()
            .apply()
    }
}

pub fn save_scene(world: &mut World) {
    let start = SystemTime::now();

    world.save(JuicePipeline).expect("Should have saved correctly");

    println!("\nTime elapsed during bevy_save saving: {:#?}\n", start.elapsed().unwrap());
}

pub fn load_scene(world: &mut World, /*mut file_state: ResMut<NextState<JuiceStates>>*/) {
    println!("LOADING!!!!!!!!!!!!!!!!!!");

    world.load(JuicePipeline).expect("Should have loaded correctly");

    //file_state.set(JuiceStates::Running);
}

pub fn get_file() -> &'static str {
    std::process::Command::new("explorer")
    .arg(".")
    .spawn()
    .unwrap();

    return "filepath";
}