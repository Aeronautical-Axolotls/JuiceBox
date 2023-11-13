use bevy::prelude::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

pub mod test;

fn main() {
    let mut juicebox: App = App::new();
	juicebox.add_plugins
	((
		DefaultPlugins, 
		test::HelloWorld, 
		
		// Debug plugins: 
		LogDiagnosticsPlugin::default(), 
		FrameTimeDiagnosticsPlugin::default(), 
	));
	juicebox.run();
}