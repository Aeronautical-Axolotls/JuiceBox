use bevy::prelude::*;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub mod sim_state_manager;
pub mod sim_physics_engine;
mod simulation;
mod util;
mod juice_renderer;
mod error;

use file_system::json_load;
use simulation::sim_state_manager;
use simulation::sim_physics_engine;
mod file_system;
// pub mod test;

fn main() {
	json_load("save_1.json");

    let mut juicebox: App = App::new();
	juicebox.add_plugins((
		DefaultPlugins, 
		sim_state_manager::SimStateManager, 
		sim_physics_engine::SimPhysicsEngine, 
		
		// Non-release plugins: 
		// test::HelloWorld, 
		// LogDiagnosticsPlugin::default(), 
		// FrameTimeDiagnosticsPlugin::default(), 
	));
	juicebox.run();
}