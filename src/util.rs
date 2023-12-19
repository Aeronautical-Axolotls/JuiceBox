use bevy::{
	math::{ Vec2, Vec4 },
	window::{ Window, WindowPlugin, MonitorSelection, WindowPosition },
	prelude::Color,
	utils::default,
	input::{ keyboard::KeyCode, Input },
	time::Time,
	transform::components::Transform,
	render::camera::{ OrthographicProjection, Camera },
	ecs::{ system::{ Res, Query }, query::With },
};
use std::{
	f32::consts::PI,
	time::SystemTime,
};

pub const WINDOW_WIDTH: f32		= 640.0;
pub const WINDOW_HEIGHT: f32	= 480.0;

/// Color definitions!
pub const JUICE_RED: Color		= Color::rgb(0.93, 0.16, 0.07);
pub const JUICE_YELLOW: Color	= Color::rgb(1.0, 0.73, 0.17);
pub const JUICE_GREEN: Color	= Color::rgb(0.48, 1.0, 0.18);
pub const JUICE_BLUE: Color		= Color::rgb(0.0, 0.25, 1.0);
pub const JUICE_SKY_BLUE: Color	= Color::rgb(0.66, 0.91, 1.0);

pub fn vector_magnitude(vector: Vec2) -> f32 {
	let mut magnitude: f32 = (vector.x * vector.x) + (vector.y * vector.y);
	magnitude = magnitude.sqrt();
	
	magnitude
}

/// Basic camera controller.
pub fn control_camera(
	keys:			Res<Input<KeyCode>>,
	time:			Res<Time>,
	mut cameras:	Query<(
		&mut Transform,
		&mut OrthographicProjection,
		With<Camera>
	)>) {
	
	// Necessary for framerate-independent camera movement.
	let delta_time: f32 = time.delta_seconds();
	
	// Move and zoom each camera.
	for (mut transform, mut projection, _) in cameras.iter_mut() {
		let speed_modifier: f32	= (keys.pressed(KeyCode::ShiftLeft) as u8) as f32;
		let camera_speed: f32	= (150.0 + (150.0 * speed_modifier)) * projection.scale * delta_time;
		let zoom_speed: f32		= (0.5 + speed_modifier) * delta_time;
		
		// Move up/down/left/right respectively.
		if keys.pressed(KeyCode::W) {
			transform.translation.y += camera_speed;
		}
		if keys.pressed(KeyCode::A) {
			transform.translation.x -= camera_speed;
		}
		if keys.pressed(KeyCode::S) {
			transform.translation.y -= camera_speed;
		}
		if keys.pressed(KeyCode::D) {
			transform.translation.x += camera_speed;
		}
		
		// Zoom in/out respectively.
		if keys.pressed(KeyCode::Q) {
			projection.scale = f32::max(projection.scale - zoom_speed, 0.05);
		}
		if keys.pressed(KeyCode::E) {
			projection.scale = f32::min(projection.scale + zoom_speed, 15.0);
		}
	}
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
pub fn generate_color_from_gradient(colors: Vec<Color>, mut value: f32) -> Color {

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
