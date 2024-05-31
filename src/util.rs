use bevy::{
    ecs::system::{NonSend, Query},
    math::{Quat, Vec2, Vec4},
    prelude::Color,
    render::camera::{Camera, OrthographicProjection},
    time::Time,
    transform::components::{GlobalTransform, Transform},
    utils::default,
    window::{MonitorSelection, Window, WindowPlugin, WindowPosition},
    winit::WinitWindows,
};
use image::RgbaImage;
use std::{
    f32::consts::{FRAC_PI_2, PI},
    time::SystemTime,
};
use winit::window::Icon;

use crate::simulation::{SimConstraints, SimGrid};

pub const WINDOW_WIDTH: f32 = 1440.0;
pub const WINDOW_HEIGHT: f32 = 1080.0;

/// Color definitions!
pub const JUICE_RED: Color = Color::rgb(0.93, 0.16, 0.07);
pub const JUICE_YELLOW: Color = Color::rgb(1.0, 0.73, 0.17);
pub const JUICE_GREEN: Color = Color::rgb(0.48, 1.0, 0.18);
pub const JUICE_BLUE: Color = Color::rgb(0.0, 0.25, 1.0);
pub const JUICE_SKY_BLUE: Color = Color::rgb(0.66, 0.91, 1.0);

/// Get the magnitude of a vector.
pub fn vector_magnitude(vector: Vec2) -> f32 {
    let mut magnitude: f32 = (vector.x * vector.x) + (vector.y * vector.y);
    magnitude = magnitude.sqrt();

    magnitude
}

/// Basic camera controller.
pub fn control_camera(
    time: &Time,
    grid: &SimGrid,
    constraints: &mut SimConstraints,
    camera: &mut (&mut Transform, &mut OrthographicProjection),
    camera_speed: f32,
    zoom_speed: f32,
    speed_mod: f32,
    horizontal_move: f32,
    vertical_move: f32,
    zoom_change: f32,
    absolute_zoom: &mut f32,
    min_zoom: f32,
    max_zoom: f32,
) {
    // Necessary for framerate-independent camera movement.
    let delta_time: f32 = time.delta_seconds();

    let min_x_position: f32 = 0.0 - ((grid.dimensions.0 / 2) * grid.cell_size) as f32;
    let min_y_position: f32 = 0.0 - ((grid.dimensions.1 / 2) * grid.cell_size) as f32;
    let max_x_position: f32 = ((grid.dimensions.0 * grid.cell_size) as f32) * 1.5;
    let max_y_position: f32 = ((grid.dimensions.1 * grid.cell_size) as f32) * 1.5;

    // Extract the transform and projection vectors for our camera.
    let transform = &mut camera.0;
    let projection = &mut camera.1;

    // Calculate the camera's true speed of movement and zooming.
    let camera_speed: f32 =
        (camera_speed + (camera_speed * speed_mod)) * projection.scale * delta_time;
    let zoom_speed: f32 = (zoom_speed + speed_mod) * delta_time;

    // Necessary data to make that camera move!
    let z_rot_rads: f32 = transform.rotation.to_euler(bevy::math::EulerRot::XYZ).2;
    let sin_rot: f32 = f32::sin(z_rot_rads);
    let cos_rot: f32 = f32::cos(z_rot_rads);

    // Handle camera movement, taking camera rotation into account.
    transform.translation.x +=
        ((horizontal_move * cos_rot) + (vertical_move * sin_rot * -1.0)) * camera_speed;
    transform.translation.y +=
        ((horizontal_move * sin_rot) + (vertical_move * cos_rot)) * camera_speed;

    // Clamp position values to within some reasonable bounds.
    transform.translation.x = f32::max(
        f32::min(transform.translation.x, max_x_position),
        min_x_position,
    );
    transform.translation.y = f32::max(
        f32::min(transform.translation.y, max_y_position),
        min_y_position,
    );

    /* Zoom in/out respectively, clamping to some reasonable bounds.  Also ensure that changes made
    by directly modifying the zoom's value via a change in zoom also affect the UI slider state.
    This is particularly important for ensuring that both keyboard and UI controls work in
    tandem. */
    *absolute_zoom += zoom_speed * zoom_change;
    *absolute_zoom = f32::max(*absolute_zoom, min_zoom);
    *absolute_zoom = f32::min(*absolute_zoom, max_zoom);
    projection.scale = 1.0 / *absolute_zoom;

    // Rotate the camera depending on the direction of gravity.
    // TODO: Make the camera rotate "in-place" as opposed to "around" the simulation.
    let gravity_angle: f32 = cartesian_to_polar(constraints.gravity).y;
    transform.rotation = Quat::from_rotation_z(gravity_angle + FRAC_PI_2);
}

/// Get the mouse cursor's position on the screen!  Returns (0.0, 0.0) if cursor position not found.
pub fn get_cursor_position(
    windows: &Query<&Window>,
    cameras: &Query<(&Camera, &GlobalTransform)>,
) -> Vec2 {
    /* TODO: Store the cursor's position every frame in some Bevy resource; maybe make it part of
    the user interaction module? */

    let window = windows.single();
    let (camera, camera_transform) = cameras.single();

    if let Some(cursor_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        let cursor_world_position = cursor_position;
        return cursor_world_position;
    }

    return Vec2::ZERO;
}

/// Gets system time in milliseconds since January 1st, 1970.
pub fn get_millis_since_epoch() -> u128 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => n.as_millis(),
        Err(_) => {
            eprintln!("Your system time is before the epoch!  RNG will not work!");
            12345678900987654321
        }
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
    let radius: f32 = polar_vector[0];
    let theta: f32 = polar_vector[1];

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
        x: sum_of_squares.sqrt(), // r = sqrt(x^2 + y^2)
        y: cy.atan2(cx),          // theta = arctan(y / x)
    };

    result
}

/** Generate a color value from a gradient between n colors based on a value between 0.0 and 1.0.
	**Color values should be provided in lowest value -> highest value order.** */
pub fn generate_color_from_gradient(colors: &Vec<Color>, mut value: f32) -> Color {
    // Clamp value and get the total number of color zones we can interpolate between.
    value = value.clamp(0.0, 1.0);
    let color_zone_count: usize = colors.len() - 1;

    // Figure out which two colors we will be interpolating between.
    let color_weight: f32 = (color_zone_count as f32) * value;
    let low_color_index: usize = color_weight.floor() as usize;
    let high_color_index: usize = color_weight.ceil() as usize;

    // Interpolate between these two colors based on value's "weight" between them.
    let lerp_weight: f32 = color_weight % 1.0;
    let weighted_color: Color = Color::from(
        Vec4::from(colors[low_color_index]).lerp(Vec4::from(colors[high_color_index]), lerp_weight),
    );

    weighted_color
}

/// Create a window plugin to add into Bevy's default plugins suite.
pub fn create_window_plugin() -> WindowPlugin {
    // First, create a nice window handle.
    let window_handle: Window = Window {
        position: WindowPosition::Centered(MonitorSelection::Primary),
        title: create_window_title("The Award-Winning JuiceBox"),
        // resizable:	false,
        // mode:		WindowMode::BorderlessFullscreen,
        ..default()
    };

    // Then, create a window plugin using the window handle!
    let window_plugin: WindowPlugin = WindowPlugin {
        primary_window: Some(window_handle),
        ..default()
    };

    window_plugin
}

/// Create a window title with a fun message appended to the title parameter.
pub fn create_window_title(title: &str) -> String {
    // Strings to be appended to the window title parameter!
    let silly_strings: [&str; 6] = [
        "Spilling encouraged!",
        "Don't cry over spilled milk!",
        "Drinking toilet water since 2023!",
        "Rivers Cuomo loves it!",
        "Domo Arigato, Mr. Roboto!",
        "Hydrate or diedrate!",
    ];

    let title_length: usize = title.len();
    let title_count: usize = silly_strings.len();

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
pub fn set_window_icon(windows: NonSend<WinitWindows>) {
    let (icon_rgba, icon_width, icon_height) = {
        // Load the JuiceBox logo icon, generating a black 16x16 image if it is not found.
        let image_result = image::open("assets/juicebox_logo_256.png");
        let image = match image_result {
            Ok(img) => img.into_rgba8(),
            Err(_error) => {
                let rgb_img: RgbaImage = RgbaImage::new(16, 16);
                rgb_img
            }
        };

        let (width, height) = image.dimensions();
        let rgba = image.into_raw();
        (rgba, width, height)
    };
    let icon = Icon::from_rgba(icon_rgba, icon_width, icon_height).unwrap();

    for window in windows.windows.values() {
        window.set_window_icon(Some(icon.clone()));
    }
}
