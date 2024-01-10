use bevy::prelude::*;
use bevy_egui::EguiPlugin;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod simulation;
mod util;
mod juice_renderer;
mod error;
mod ui;

use simulation::sim_state_manager;
use simulation::sim_physics_engine;
use ui::ui_base;
// pub mod test;

fn main() {
    let mut juicebox: App = App::new();
	juicebox.add_plugins((
		DefaultPlugins.set(util::create_window_plugin()),
		sim_state_manager::SimStateManager,
		sim_physics_engine::SimPhysicsEngine,
		juice_renderer::JuiceRenderer,
		EguiPlugin,

		// Non-release plugins:
		// test::HelloWorld,
		// LogDiagnosticsPlugin::default(),
		// FrameTimeDiagnosticsPlugin::default(),
	));
	juicebox.add_systems(Update,ui_base);
	juicebox.run();
}
