use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::{Window, WindowPlugin};
use bevy::core_pipeline::prelude::ClearColor;
use bevy::render::RenderApp;
use crate::util::generate_random_usize;

pub const WINDOW_WIDTH: f32		= 640.0;
pub const WINDOW_HEIGHT: f32	= 480.0;

/// Color definitions!
pub const JUICE_RED: Color		= Color::rgb(0.93, 0.16, 0.07);
pub const JUICE_YELLOW: Color	= Color::rgb(1.0, 0.73, 0.17);
pub const JUICE_GREEN: Color	= Color::rgb(0.48, 1.0, 0.18);
pub const JUICE_BLUE: Color		= Color::rgb(0.66, 0.91, 1.0);

pub struct JuiceRenderer;
impl Plugin for JuiceRenderer {

	fn build(&self, app: &mut App) {
		app.insert_resource(ClearColor(JUICE_BLUE));

		app.add_systems(Startup, setup_renderer);

		let mut render_app = app.sub_app_mut(RenderApp);
		// render_app.add_systems(Render, something_probably_goes_here_but_idk_what_yet);
	}
}

/// Custom rendering pipeline initialization.
fn setup_renderer(
	mut commands:	Commands,
	mut meshes:		ResMut<Assets<Mesh>>,
	mut materials:	ResMut<Assets<ColorMaterial>>) {

	// Spawn a camera to view our simulation world!
	commands.spawn(Camera2dBundle::default());

	// Spawn test particle!
	let particle_color_material: ColorMaterial = ColorMaterial::from(
		generate_color_from_gradient(JUICE_GREEN, JUICE_YELLOW, JUICE_RED, 0.5)
	);
	commands.spawn(MaterialMesh2dBundle {
		mesh:		meshes.add(shape::Circle::new(10.0).into()).into(),
		material:	materials.add(particle_color_material),
		transform:	Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
		..default()
	});
}

/// Generate a color value from a gradient between 3 colors based on a value between 0.0 and 1.0.
pub fn generate_color_from_gradient(
	low_color:	Color,
	mid_color:	Color,
	high_color:	Color,
	mut value:	f32) -> Color {

	value = value.clamp(0.0, 1.0);
	let mut weighted_color: Color;

	/* We only need to blend between the colors whose range we are in (either low_color and
		mid_color, or mid_color and high_color). */
	if value < 0.5 {
		let value_compliment: f32 = 0.5 - value;
		weighted_color = mid_color * (value * 2.0);
		weighted_color += low_color * (value_compliment * 2.0);
	} else {
		value -= 0.5;
		let value_compliment: f32 = 0.5 - value;
		weighted_color = high_color * (value * 2.0);
		weighted_color += mid_color * (value_compliment * 2.0);
	}

	weighted_color
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
