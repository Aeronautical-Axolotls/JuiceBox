use bevy::{
	ecs::{ entity::Entity, query::With, system::{ Commands, NonSend, Query, Res, ResMut } }, gizmos::gizmos::Gizmos, input::{ keyboard::KeyCode, mouse::MouseButton, Input }, math::{ Vec2, Vec4 }, prelude::Color, render::camera::{ Camera, OrthographicProjection }, time::Time, transform::components::{GlobalTransform, Transform}, utils::default, window::{ MonitorSelection, Window, WindowPlugin, WindowPosition }, winit::WinitWindows
};
use winit::window::Icon;
use std::{
	f32::consts::PI,
	time::SystemTime,
};

use crate::{juice_renderer::draw_vector_arrow, simulation::{sim_state_manager, SimConstraints, SimGrid, SimGridCellType, SimParticle}, test::test_state_manager};

pub const WINDOW_WIDTH: f32		= 640.0;
pub const WINDOW_HEIGHT: f32	= 480.0;

/// Color definitions!
pub const JUICE_RED: Color		= Color::rgb(0.93, 0.16, 0.07);
pub const JUICE_YELLOW: Color	= Color::rgb(1.0, 0.73, 0.17);
pub const JUICE_GREEN: Color	= Color::rgb(0.48, 1.0, 0.18);
pub const JUICE_BLUE: Color		= Color::rgb(0.0, 0.25, 1.0);
pub const JUICE_SKY_BLUE: Color	= Color::rgb(0.66, 0.91, 1.0);

/// Get the magnitude of a vector.
pub fn vector_magnitude(vector: Vec2) -> f32 {
	let mut magnitude: f32 = (vector.x * vector.x) + (vector.y * vector.y);
	magnitude = magnitude.sqrt();

	magnitude
}

/// Debugging state controller.
pub fn debug_state_controller(
	mut commands:		Commands,
	keys:				Res<Input<KeyCode>>,
	mouse:				Res<Input<MouseButton>>,
	windows:			Query<&Window>,
	cameras:			Query<(&Camera, &GlobalTransform)>,
	mut constraints:	ResMut<SimConstraints>,
	mut grid:			ResMut<SimGrid>,
	mut particles:		Query<(Entity, &mut SimParticle)>) {
	
	// Reset simulation when we press R.
	if keys.just_pressed(KeyCode::R) {
		
		/* BUG: The more times you reset the simulation the slower it runs.  I suspect this 
			is caused by either the sprites not being properly unloaded when each particle 
			component is despawned, OR the fact that we are repeatedly creating new grid and 
			constraint resources, effectively leaking memory.  Not sure which; figure it out! */
		crate::simulation::reset_simulation_to_default(
			&mut commands,
			constraints.as_mut(),
			grid.as_mut(),
			&mut particles
		);
		test_state_manager::construct_test_simulation_layout(
			constraints.as_mut(),
			grid.as_mut(),
			commands
		);
	}
	
	// Rotate/scale gravity when we press the arrow keys.
	let gravity_rotation: i8	= keys.pressed(KeyCode::Right) as i8 - keys.pressed(KeyCode::Left) as i8;
	let gravity_magnitude: i8	= keys.pressed(KeyCode::Up) as i8 - keys.pressed(KeyCode::Down) as i8;
	let mut polar_gravity: Vec2	= cartesian_to_polar(constraints.gravity);
	polar_gravity.x				+= 200.0 * gravity_magnitude as f32 * constraints.timestep;
	polar_gravity.y				+= 4.0 * gravity_rotation as f32 * constraints.timestep;
	
	// Limit the magnitude of the vector to prevent ugly behavior near 0.0.
	polar_gravity.x				= f32::max(0.0, polar_gravity.x);
	constraints.gravity			= polar_to_cartesian(polar_gravity);
	
	// Place/remove grid cells if the mouse is clicked on a cell.
	let should_place_cell: bool		= mouse.pressed(MouseButton::Left);
	let should_remove_cell: bool	= mouse.pressed(MouseButton::Right);
	
	if should_place_cell {
		let cursor_position: Vec2	= get_cursor_position(&windows, &cameras);
		let cell_coordinates: Vec2	= grid.get_cell_coordinates_from_position(&cursor_position);
		let _ = grid.set_grid_cell_type(
			cell_coordinates.x as usize,
			cell_coordinates.y as usize,
			SimGridCellType::Solid
		);
		
		// let lookup_index: usize = grid.get_lookup_index(cell_coordinates);
		// grid.remove_particles_in_cell(lookup_index);
		
	} else if should_remove_cell {
		let cursor_position: Vec2	= get_cursor_position(&windows, &cameras);
		let cell_coordinates: Vec2	= grid.get_cell_coordinates_from_position(&cursor_position);
		let _ = grid.set_grid_cell_type(
			cell_coordinates.x as usize,
			cell_coordinates.y as usize,
			SimGridCellType::Air
		);
	}
}

/// Basic camera controller.
pub fn control_camera(
	keys:			Res<Input<KeyCode>>,
	time:			Res<Time>,
	grid:			Res<SimGrid>,
	mut cameras:	Query<(
		&mut Transform,
		&mut OrthographicProjection,
		With<Camera>
	)>) {

	// Necessary for framerate-independent camera movement.
	let delta_time: f32 = time.delta_seconds();

	let min_x_position: f32	= 0.0 - ((grid.dimensions.0 / 2) * grid.cell_size) as f32;
	let min_y_position: f32	= 0.0 - ((grid.dimensions.1 / 2) * grid.cell_size) as f32;
	let max_x_position: f32	= ((grid.dimensions.0 * grid.cell_size) as f32) * 1.5;
	let max_y_position: f32	= ((grid.dimensions.1 * grid.cell_size) as f32) * 1.5;

	// TODO: Factor in the number of grid cells with this calculation.
	let min_zoom: f32		= (grid.cell_size as f32) * 0.0075;
	let max_zoom: f32		= (grid.cell_size as f32) / 2.0;

	// Move and zoom each camera.
	for (mut transform, mut projection, _) in cameras.iter_mut() {
		let speed_mod: f32		= (keys.pressed(KeyCode::ShiftLeft) as u8) as f32;
		let camera_speed: f32	= (150.0 + (150.0 * speed_mod)) * projection.scale * delta_time;
		let zoom_speed: f32		= (0.5 + speed_mod) * delta_time;

		// Move up/down/left/right respectively.
		if keys.pressed(KeyCode::W) {
			transform.translation.y = f32::min(
				transform.translation.y + camera_speed,
				max_y_position
			);
		}
		if keys.pressed(KeyCode::A) {
			transform.translation.x = f32::max(
				transform.translation.x - camera_speed,
				min_x_position
			);
		}
		if keys.pressed(KeyCode::S) {
			transform.translation.y = f32::max(
				transform.translation.y - camera_speed,
				min_y_position
			);
		}
		if keys.pressed(KeyCode::D) {
			transform.translation.x = f32::min(
				transform.translation.x + camera_speed,
				max_x_position
			);
		}

		// Zoom in/out respectively.
		if keys.pressed(KeyCode::Q) {
			projection.scale = f32::max(projection.scale - zoom_speed, min_zoom);
		}
		if keys.pressed(KeyCode::E) {
			projection.scale = f32::min(projection.scale + zoom_speed, max_zoom);
		}
	}
}

/// Get the mouse cursor's position on the screen!  Returns (0.0, 0.0) if cursor position not found.
pub fn get_cursor_position(
	windows: &Query<&Window>,
	cameras: &Query<(&Camera, &GlobalTransform)>) -> Vec2 {

	/* TODO: Store the cursor's position every frame in some Bevy resource; maybe make it part of
		the user interaction module? */

	let window = windows.single();
	let (camera, camera_transform) = cameras.single();

	if let Some(cursor_position) = window.cursor_position()
		.and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) {

		let cursor_world_position = cursor_position;
		return cursor_world_position;
	}

	return Vec2::ZERO;
}

/// Gets system time in milliseconds since January 1st, 1970.
pub fn get_millis_since_epoch() -> u128 {
	match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
		Ok(n)	=> n.as_millis(),
		Err(_)	=> {
			eprintln!("Your system time is before the epoch!  RNG will not work!");
			12345678900987654321
		},
	}
}

/// Generates a pseudorandom usize; based on theory found in "Xorshift RNGs" by George Marsaglia.
pub fn generate_random_usize(seed: usize) -> usize {
	let mut rand: usize = get_millis_since_epoch() as usize;
	rand += seed;

	rand ^= rand << 13;
	rand ^= rand >> 7;
	rand ^= rand << 17;
	rand
}

/// Generates a pseudorandom u32; based on theory found in "Xorshift RNGs" by George Marsaglia.
pub fn generate_random_u32(seed: u32) -> u32 {
	let mut rand: u32 = get_millis_since_epoch() as u32;
	rand += seed;

	rand ^= rand << 13;
	rand ^= rand >> 17;
	rand ^= rand << 5;
	rand
}

/// Converts degrees to radians; returns radians.
pub fn degrees_to_radians(degrees: f32) -> f32 {
	let radians: f32 = degrees * (PI / 180.0);
	radians
}

/// Converts radians to degrees; returns degrees.
pub fn radians_to_degrees(radians: f32) -> f32 {
	let degrees: f32 = radians * (180.0 / PI);
	degrees
}

/** Converts a polar vector with direction and magnitude into a cartesian vector with x and y
	components; returns said cartesian vector.  **Note: polar vectors are of the form
	(magnitude, angle-in-radians).** */
pub fn polar_to_cartesian(polar_vector: Vec2) -> Vec2 {
	let radius:	f32 = polar_vector[0];
	let theta:	f32 = polar_vector[1];

	let result: Vec2 = Vec2 {
		x: radius * theta.cos(),
		y: radius * theta.sin(),
	};

	result
}

/** Converts a cartesian vector with x and y components into a polar vector with direction and
	magnitude; returns said polar vector.  **Note: polar vectors are of the form
	(magnitude, angle-in-radians).** */
pub fn cartesian_to_polar(cartesian_vector: Vec2) -> Vec2 {
	let cx: f32 = cartesian_vector[0];
	let cy: f32 = cartesian_vector[1];

	let sum_of_squares: f32 = (cx * cx) + (cy * cy);

	let result: Vec2 = Vec2 {
		x: sum_of_squares.sqrt(),	// r = sqrt(x^2 + y^2)
		y: cy.atan2(cx),			// theta = arctan(y / x)
	};

	result
}

/** Generate a color value from a gradient between n colors based on a value between 0.0 and 1.0.
	**Color values should be provided in lowest value -> highest value order.** */
pub fn generate_color_from_gradient(colors: &Vec<Color>, mut value: f32) -> Color {

	// Clamp value and get the total number of color zones we can interpolate between.
	value						= value.clamp(0.0, 1.0);
	let color_zone_count: usize	= colors.len() - 1;

	// Figure out which two colors we will be interpolating between.
	let color_weight: f32		= (color_zone_count as f32) * value;
	let low_color_index: usize	= color_weight.floor() as usize;
	let high_color_index: usize	= color_weight.ceil() as usize;

	// Interpolate between these two colors based on value's "weight" between them.
	let lerp_weight: f32		= color_weight % 1.0;
	let weighted_color: Color = Color::from(
		Vec4::from(colors[low_color_index]).lerp(
			Vec4::from(colors[high_color_index]),
			lerp_weight
		)
	);

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

/// Sets the window icon for the app window(s).
pub fn set_window_icon(windows: NonSend<WinitWindows>)
{
	let (icon_rgba, icon_width, icon_height) = {
		let image = image::open("assets/juicebox_logo_256.png")
			.expect("Failed to open icon!")
			.into_rgba8();
		let (width, height)	= image.dimensions();
		let rgba			= image.into_raw();
		(rgba, width, height)
	};
	let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

	for window in windows.windows.values() {
		window.set_window_icon(Some(icon.clone()));
	}
}
