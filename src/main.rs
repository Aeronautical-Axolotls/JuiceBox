use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_save::SavePlugin;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub mod simulation;
pub mod util;
pub mod juice_renderer;
pub mod error;
pub mod file_system;
// pub mod test and ui;

pub mod test;
pub mod ui;

use simulation::{Simulation, SimGrid};
use ui::ui_base;



fn main() {
    let mut juicebox: App = App::new();

	juicebox.add_plugins((
		DefaultPlugins
		.set(util::create_window_plugin())
		.set(AssetPlugin{
			watch_for_changes_override: Some(true),
			..Default::default()
		}),
		simulation::Simulation,
		juice_renderer::JuiceRenderer,
		EguiPlugin,
		SavePlugin,

		// Non-release plugins:
		// LogDiagnosticsPlugin::default(),
		// FrameTimeDiagnosticsPlugin::default(),
	));

	juicebox.add_systems(Update, ui_base);

	juicebox.add_systems(Update, util::control_camera);

	juicebox.run();
}

