use bevy::prelude::*;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub mod sim_state_manager;
pub mod sim_physics_engine;
mod error;
// pub mod test;

fn main() {
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
