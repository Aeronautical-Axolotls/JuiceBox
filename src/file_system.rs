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


use bevy::{prelude::*, tasks::IoTaskPool, utils::Duration};
use std::{fs::File, io::Write};
use bevy::ecs::query::*;

use crate::simulation::{SimParticle, SimGrid, SimConstraints};
use crate::juice_renderer;

//#[derive(bevy::ecs::query::QueryFilter)]
struct EntitySaveFilter<> {

}
use super::*;

pub fn save_scene(world: &mut World) {
    println!("Saving Scene...");

    /*
    let mut scene_world = World::new();
    let type_registry = world.resource::<AppTypeRegistry>().clone();
    scene_world.insert_resource(type_registry);
    let mut test_constraints = SimConstraints::from_world(world);
    let mut test_grid = SimGrid::from_world(world);
    let mut test_particle = SimParticle::from_world(world);
    scene_world.insert_resource(test_constraints);
    scene_world.insert_resource(test_grid);
    scene_world.spawn(test_particle);
    let scene = DynamicScene::from_world(&scene_world);
    let type_registry = world.resource::<AppTypeRegistry>();
    let serialized_scene = scene.serialize_ron(type_registry).unwrap();
    //info!("{}", serialized_scene);

    #[cfg(not(target_arch = "wasm32"))]
    IoTaskPool::get()
        .spawn(async move {
            // Write the scene RON data to file
            File::create(format!("assets/scenes/test_save_1.scn.ron"))
                .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                .expect("Error while writing scene to file");
        })
        .detach();
    */


    // Method #3

    let mut path = format!("assets/scenes/test_save_3_2.scn.ron");

    let entities_vec: Vec<Entity> = world
    //.query_filtered::<Entity, With<Savable>>()
    .query_filtered::<Entity, With<SimParticle>>()
    .iter(world)
    .collect();

    println!("{:#?}", entities_vec);

    //let x = entities_vec.into_iter();

    let mut scene_builder = DynamicSceneBuilder::from_world(world);
    //let mut new_scene: DynamicScene = scene_builder.allow_all_resources().extract_resources().extract_entities(entities_vec.into_iter()).build(); // Look into [allow_resource]
    let mut test_scene: DynamicScene = scene_builder.extract_entities(entities_vec.into_iter()).build();

    println!("{:#?}", test_scene.serialize_ron(world.resource::<AppTypeRegistry>()));

    match test_scene.serialize_ron(world.resource::<AppTypeRegistry>()) {
        Ok(serialized_scene) => match File::create(&path) {
            Ok(mut file) => match file.write_all(serialized_scene.as_bytes()) {
                Ok(()) => info!("save successful: {path:?}"),
                Err(why) => error!("save failed: {why:?}"),
            },
            Err(why) => {
                error!("file creation failed: {why:?}");
            }
        },
        Err(why) => {
            error!("serialization failed: {why:?}");
        }
    }

    /*
    let entities: Vec<Entity> = match mode {
        SaveMode::Filtered => world
            .query_filtered::<Entity>()
            .iter(world)
            .collect(),
        SaveMode::Dump => world.iter_entities().collect(),
    };
    */

    //println!("\n\n THE WORLD COMPONENT LIST \n\n");
    //println!("{:#?}", world.components());

    //println!("{:#?}", world.entities());
    
    //let mut iter_test = world.iter_entities();


    //println!("{:#?}", world.inspect_entity(iter_test.next().unwrap().id()));
    //println!("{:#?}", world.inspect_entity(iter_test.next().unwrap().id()));
    //println!("{:#?}", world.inspect_entity(iter_test.next().unwrap().id()));
    
    /*
    println!("Time: {:?}", world.resource::<Time>());

    let scene = DynamicScene::from_world(world);
    let type_registry = world.resource::<AppTypeRegistry>();

    println!("{:?}", world.entities());

    let serialized_scene = scene.serialize_ron(type_registry).unwrap();
    println!("Serialized Scene:\n\n{}", serialized_scene);
    */
    

    //let dynamic_scene = from_world(world);
}

pub fn load_scene(mut commands: Commands, asset_server: Res<AssetServer>) {
    println!("Loading Scene...");
    commands.spawn(DynamicSceneBundle {scene: asset_server.load("scenes/test_load_3.scn.ron"),..default()});
    /*
    let particle: Entity = commands.spawn(
		SimParticle {
			position:	Vec2 {x: 20.0, y: 20.0},
			velocity:	Vec2 {x: 0.0, y: 0.0},
		}
	).id();
    */

	// IMPORTANT: Links a sprite to each particle for rendering.
	//juice_renderer::link_particle_sprite(&mut commands, particle);
}

pub fn get_file() -> &'static str {
    std::process::Command::new("explorer")
    .arg(".")
    .spawn()
    .unwrap();

    return "filepath";
}