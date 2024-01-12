use bevy::prelude::*;
use bevy_egui::EguiPlugin;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub mod simulation;
pub mod util;
pub mod juice_renderer;
pub mod error;
// pub mod test and ui;

pub mod test;
pub mod ui;

use simulation::Simulation;
use ui::ui_base;



fn main() {
    let mut juicebox: App = App::new();

	juicebox.add_plugins((
		DefaultPlugins.set(util::create_window_plugin()),
		simulation::Simulation,
		juice_renderer::JuiceRenderer,
		EguiPlugin,

		// Non-release plugins:
		// LogDiagnosticsPlugin::default(),
		// FrameTimeDiagnosticsPlugin::default(),
	));
	
	juicebox.add_systems(Startup, util::set_window_icon);
	
	juicebox.add_systems(Update, ui_base);
	juicebox.add_systems(Update, util::control_camera);

	juicebox.run();
}
