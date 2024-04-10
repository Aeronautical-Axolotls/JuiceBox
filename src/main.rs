use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub mod simulation;
pub mod util;
pub mod juice_renderer;
pub mod error;
pub mod test;
pub mod events;
pub mod ui;

use util::debug_state_controller;

fn main() {

    let mut juicebox: App = App::new();

	juicebox.add_plugins((
		DefaultPlugins.set(util::create_window_plugin()),
		simulation::Simulation,
		juice_renderer::JuiceRenderer,
        ui::JuiceUI,
		EguiPlugin,

		// Non-release plugins:
		LogDiagnosticsPlugin::default(),
		FrameTimeDiagnosticsPlugin::default(),
	));

	juicebox.add_systems(Startup, util::set_window_icon);
	juicebox.run();
}
