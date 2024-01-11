use bevy::prelude::*;
use bevy_egui::EguiPlugin;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

mod simulation;
mod util;
mod juice_renderer;
mod error;
// pub mod test and ui;

mod test;
mod ui;

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

	juicebox.add_systems(Update,ui_base);

	juicebox.add_systems(Update, util::control_camera);

	juicebox.run();
}
