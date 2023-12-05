use bevy::prelude::*;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod sim_state_manager;
mod sim_physics_engine;
mod juice_math;
mod juice_renderer;
mod error;
// pub mod test;

fn main() {
    let mut juicebox: App = App::new();
	juicebox.add_plugins((
		DefaultPlugins.set(juice_renderer::create_window_plugin()),
		juice_renderer::JuiceRenderer,
		sim_state_manager::SimStateManager,
		sim_physics_engine::SimPhysicsEngine,

		// Non-release plugins:
		// test::HelloWorld,
		// LogDiagnosticsPlugin::default(),
		// FrameTimeDiagnosticsPlugin::default(),
	));
	juicebox.run();
}