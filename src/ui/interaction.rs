use std::borrow::BorrowMut;
use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::events::{PlayPauseStepEvent, ResetEvent, UseToolEvent};
use crate::util::*;
use crate::ui::UIStateManager;
use crate::simulation::{change_gravity, SimConstraints, SimGrid};

use super::SimTool;

/// Debugging state controller.
pub fn handle_input(
	mut constraints:	ResMut<SimConstraints>,
	keys:				Res<Input<KeyCode>>,
	mouse:				Res<Input<MouseButton>>,
	windows:			Query<&Window>,
	cameras:			Query<(&Camera, &GlobalTransform)>,
	mut ui_state:     	ResMut<UIStateManager>,
    mut ev_reset:       EventWriter<ResetEvent>,
    mut ev_tool_use:	EventWriter<UseToolEvent>,
	mut ev_pause:		EventWriter<PlayPauseStepEvent>) {

	let left_mouse_pressed: bool	= mouse.pressed(MouseButton::Left);
	let right_mouse_pressed: bool	= mouse.pressed(MouseButton::Right);
	let left_right: f32				= (keys.pressed(KeyCode::Right) as i8 - keys.pressed(KeyCode::Left) as i8) as f32;
	let up_down: f32				= (keys.pressed(KeyCode::Up) as i8 - keys.pressed(KeyCode::Down) as i8) as f32;
	let r_key_pressed:bool			= keys.just_pressed(KeyCode::R);
	let f_key_pressed:bool			= keys.just_pressed(KeyCode::F);
	let space_pressed:bool			= keys.just_pressed(KeyCode::Space);

	// Reset simulation when we press R or when UI button is pressed.
	if r_key_pressed { ev_reset.send(ResetEvent); return; }
	if ui_state.reset { ui_state.reset = false; ev_reset.send(ResetEvent); return; }
	// Pause/unpause the simulation if Space is pressed.
	if space_pressed { ev_pause.send(PlayPauseStepEvent::new(false)); return; }
	// Step once if the F key is pressed.
	if f_key_pressed { ev_pause.send(PlayPauseStepEvent::new(true)); return; }
	ui_state.is_paused = constraints.is_paused;

	// Handle tool usage for both mouse buttons.
	if left_mouse_pressed || right_mouse_pressed {

		let mouse_button: MouseButton;
		if left_mouse_pressed	{ mouse_button = MouseButton::Left; }
		else					{ mouse_button = MouseButton::Right; }

        ev_tool_use.send(UseToolEvent::new(
			ui_state.selected_tool,
			get_cursor_position(&windows, &cameras),
			Some(mouse_button)
		));
	}

	/* Rotate/scale gravity when we press the arrow keys.  First, set the simulation's gravity to
		that which is found in the UI.  Then, change the simulation's gravity values based on
		keyboard input.  Finally, convert the modified gravity value from the simulation back into
		values that the UI can display.  This allows for keyboard and UI slider control to work
		in tandem. */
	constraints.gravity = polar_to_cartesian(Vec2 {
		x: ui_state.gravity_magnitude * ui_state.gravity_magnitude * 4.0,
		y: degrees_to_radians(ui_state.gravity_direction) - PI
	});
	change_gravity(constraints.as_mut(), up_down * 6.0, left_right);
	let polar_gravity			= cartesian_to_polar(constraints.gravity);
	ui_state.gravity_magnitude	= f32::sqrt(polar_gravity.x / 4.0);
	ui_state.gravity_direction	= radians_to_degrees(polar_gravity.y + PI);
}

/// Handle all user input as it relates to the camera!
pub fn handle_camera_input(
	mut constraints:		ResMut<SimConstraints>,
	grid:					Res<SimGrid>,
	time:					Res<Time>,
	keys:					Res<Input<KeyCode>>,
	mouse:					Res<Input<MouseButton>>,
	mut mut_cameras:		Query<(&mut Transform, &mut OrthographicProjection, With<Camera>)>,
	mut ui_state:     		ResMut<UIStateManager>,
	mut ev_mouse_motion:	EventReader<MouseMotion>) {

	// All user input that camera controlling is concerned with.
	let left_mouse_pressed: bool		= mouse.pressed(MouseButton::Left);
	let mut camera_horizontal_move: f32	= (keys.pressed(KeyCode::D) as i8 - keys.pressed(KeyCode::A) as i8) as f32;
	let mut camera_vertical_move: f32	= (keys.pressed(KeyCode::W) as i8 - keys.pressed(KeyCode::S) as i8) as f32;
	let camera_zoom_change: f32			= (keys.pressed(KeyCode::E) as i8 - keys.pressed(KeyCode::Q) as i8) as f32;
	let camera_speed_mod: f32			= (keys.pressed(KeyCode::ShiftLeft) as u8) as f32;

	/* Define camera_speed here so we can modify its values for dragging the camera (zoom_speed
		also defined here for consistency and aesthetics). */
	let mut camera_speed: f32	= 150.0;
	let zoom_speed: f32			= 1.0;

	// Extract the camera from our Query<> to control it.
	let camera_query = &mut mut_cameras.single_mut();
	let mut camera = (camera_query.0.as_mut(), camera_query.1.as_mut());

	// If we have clicked the left mouse button and the camera tool is selected, move the camera!
	if left_mouse_pressed {
		match ui_state.selected_tool {
			SimTool::Camera => {
				for motion in ev_mouse_motion.read() {
					camera_horizontal_move	= -1.0 * motion.delta.x;
					camera_vertical_move	= motion.delta.y;
					camera_speed			= 100.0;
				}
			},
			_ => {},
		}
	}

	// Control the camera based on user input arguments.
	control_camera(
		&time,
		&grid,
		&mut constraints,
		&mut camera,
		camera_speed,
		zoom_speed,
		camera_speed_mod,
		camera_horizontal_move,
		camera_vertical_move,
		camera_zoom_change,
		&mut ui_state.zoom_slider,
		0.5,// (grid.cell_size as f32) * 0.0075,
		5.0// (grid.cell_size as f32) / 2.0
	);
}

/// Handles incoming events from the UI
pub fn change_cursor_icon(
    mut ev_reset:       EventReader<ResetEvent>,
    mut ev_tool_use:    EventReader<UseToolEvent>,
	mut windows:		Query<&mut Window>,
	ui_state:			Res<UIStateManager>) {

	// Set the default cursor icon.
	let mut window = windows.single_mut();
	window.cursor.icon = CursorIcon::Default;

    // If there is a reset event sent, set the icon to loading!
    for _ in ev_reset.read() {
		window.cursor.icon = CursorIcon::Wait;
		return;
    }

	// Change the cursor icon depending on the currently selected tool.
	println!("{:?}", ui_state.selected_tool);
	match ui_state.selected_tool {
		SimTool::Select			=> window.cursor.icon = CursorIcon::Default,
		SimTool::Camera			=> window.cursor.icon = CursorIcon::Move,
		SimTool::Zoom			=> window.cursor.icon = CursorIcon::ZoomIn,
		SimTool::Gravity		=> window.cursor.icon = CursorIcon::Default,
		SimTool::Grab			=> window.cursor.icon = CursorIcon::Hand,
		SimTool::AddFluid		=> window.cursor.icon = CursorIcon::Hand,
		SimTool::RemoveFluid	=> window.cursor.icon = CursorIcon::Hand,
		SimTool::AddWall		=> window.cursor.icon = CursorIcon::Hand,
		SimTool::RemoveWall		=> window.cursor.icon = CursorIcon::Hand,
		SimTool::AddDrain		=> window.cursor.icon = CursorIcon::Hand,
		SimTool::RemoveDrain	=> window.cursor.icon = CursorIcon::Hand,
		SimTool::AddFaucet		=> window.cursor.icon = CursorIcon::Hand,
		SimTool::RemoveFaucet	=> window.cursor.icon = CursorIcon::Hand,
	}

    // For tools that need an icon change when in use:
    for tool_use in ev_tool_use.read() {
        match tool_use.tool {
            SimTool::Grab	=> window.cursor.icon = CursorIcon::Grabbing,
			_				=> {},
        }
    }
}