use bevy::prelude::*;
use bevy::window::{Window, WindowPlugin};

use crate::juice_math::generate_random_usize;

pub struct JuiceRenderer;
impl Plugin for JuiceRenderer {
	
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup_renderer);
	}
}

/// Custom rendering pipeline initialization.
fn setup_renderer(mut _commands: Commands) {
	
}

pub struct ParticleSystem {
	pub rendered_texture: Handle<Image>, 
}

/// Create a window plugin to add into Bevy's default plugins suite.
pub fn create_window_plugin() -> WindowPlugin {
	let window_handle: Window = Window {
		position:	WindowPosition::Centered(MonitorSelection::Primary), 
		resolution:	(640.0, 480.0).into(), 
		title:		create_window_title("JuiceBox"), 
		..default()
	};
	
	let window_plugin: WindowPlugin = WindowPlugin {
		primary_window: Some(window_handle), 
		..default()
	};
	
	window_plugin
}

/// Create a window title with a fun message appended to the title parameter.
pub fn create_window_title(title: &str) -> String {
	// Strings to be appended to the window title parameter!
	let silly_strings: [&str; 10]	= [
		"Spilling encouraged!", 
		"Don't cry over spilled milk!", 
		"Rolling in the deep!", 
		"Don't drink the toilet water!", 
		"We're not fans of dry humor...", 
		"Rivers Cuomo loves it!", 
		"Cry me a river!", 
		"Styx and stones!", 
		"Hydrate or diedrate!", 
		"Liquid fun!", 
	];
	
	let title_length: usize	= title.len();
	let title_count: usize	= silly_strings.len();
	
	// Choose a random tagline for the window title, but prefer the first option.
	let mut random_index: i8 = (generate_random_usize(title_length) % (title_count * 2)) as i8;
	random_index -= title_count as i8;
	if random_index < 0 {
		random_index = 0;
	}
	
	// Append the randomely chosen tagline to the window title parameter.
	let tagline: &str = silly_strings[random_index as usize];
	let mut spruced_title: String = title.to_string().to_owned();
	spruced_title.push_str(" ~ ");
	spruced_title.push_str(tagline);
	
	spruced_title
}