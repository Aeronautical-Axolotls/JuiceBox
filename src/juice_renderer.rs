use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::{Window, WindowPlugin};
use bevy::core_pipeline::prelude::ClearColor;
use bevy::render::RenderApp;
use crate::juice_math::generate_random_usize;
use crate::sim_state_manager::SimParticles;

pub const WINDOW_WIDTH: f32		= 640.0;
pub const WINDOW_HEIGHT: f32	= 480.0;

/// Color definitions!
pub const JUICE_BLUE: Color		= Color::rgb(0.66, 0.91, 1.0);
pub const JUICE_AMBER: Color	= Color::rgb(1.0, 0.73, 0.17);

pub struct JuiceRenderer;
impl Plugin for JuiceRenderer {
	
	fn build(&self, app: &mut App) {
		app.insert_resource(ClearColor(JUICE_AMBER));
		
		app.add_systems(Startup, setup_renderer);
		
		let mut render_app = app.sub_app_mut(RenderApp);
		// render_app.add_systems(Render, something_probably_goes_here_but_idk_what_yet);
	}
}

/// Custom rendering pipeline initialization.
fn setup_renderer(
	mut commands:	Commands,
	mut meshes:		ResMut<Assets<Mesh>>,
	mut materials:	ResMut<Assets<ColorMaterial>>, 
	mut particles:	ResMut<SimParticles>) {
	
	// Spawn a camera to view our simulation world!
	commands.spawn(Camera2dBundle::default());
	
	// Spawn test particle!
	commands.spawn(MaterialMesh2dBundle {
		mesh:		meshes.add(shape::Circle::new(50.0).into()).into(),
		material:	materials.add(ColorMaterial::from(JUICE_BLUE)),
		transform:	Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
		..default()
	});
}

/// Create a window plugin to add into Bevy's default plugins suite.
pub fn create_window_plugin() -> WindowPlugin {
	let window_handle: Window = Window {
		position:	WindowPosition::Centered(MonitorSelection::Primary), 
		resolution:	(WINDOW_WIDTH, WINDOW_HEIGHT).into(), 
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
	let silly_strings: [&str; 6]	= [
		"Spilling encouraged!", 
		"Don't cry over spilled milk!", 
		"Drinking toilet water since 2023!", 
		"Rivers Cuomo loves it!", 
		"Domo Arigato, Mr. Roboto!", 
		"Hydrate or diedrate!", 
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