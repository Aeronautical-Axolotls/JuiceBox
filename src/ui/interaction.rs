use std::borrow::BorrowMut;

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::events::{ResetEvent, UseToolEvent};
use crate::util::*;
use crate::ui::UIStateManager;
use crate::simulation::{change_gravity, SimConstraints, SimGrid};

use super::SimTool;

/// Debugging state controller.
pub fn handle_input(
	mut constraints:	ResMut<SimConstraints>,
	grid:				Res<SimGrid>,
	time:				Res<Time>,
	keys:				Res<Input<KeyCode>>,
	mouse:				Res<Input<MouseButton>>,
	windows:			Query<&Window>,
	cameras:			Query<(&Camera, &GlobalTransform)>,
	mut mut_cameras:	Query<(&mut Transform, &mut OrthographicProjection, With<Camera>)>,
	mut ui_state:     		ResMut<UIStateManager>,

    mut ev_reset:       	EventWriter<ResetEvent>,
    mut ev_tool_use:	    EventWriter<UseToolEvent>,
	mut ev_mouse_motion:	EventReader<MouseMotion>) {

	// Reset simulation when we press R.
	if keys.just_pressed(KeyCode::R) {

        ev_reset.send(ResetEvent);

		// crate::simulation::reset_simulation_to_default(
		// 	&mut commands,
		// 	constraints.as_mut(),
		// 	grid.as_mut(),
		// 	&mut particles
		// );
		// test_state_manager::construct_test_simulation_layout(
		// 	constraints.as_mut(),
		// 	grid.as_mut(),
		// 	commands
		// );

		return;
	}

	// Mouse input!
	let left_mouse_pressed: bool = mouse.pressed(MouseButton::Left);
	let right_mouse_pressed: bool = mouse.pressed(MouseButton::Right);

	// Camera controller input.
	let mut camera_horizontal_move: f32	= (keys.pressed(KeyCode::D) as i8 - keys.pressed(KeyCode::A) as i8) as f32;
	let mut camera_vertical_move: f32	= (keys.pressed(KeyCode::W) as i8 - keys.pressed(KeyCode::S) as i8) as f32;
	let camera_zoom_change: f32			= (keys.pressed(KeyCode::E) as i8 - keys.pressed(KeyCode::Q) as i8) as f32;

	let camera_speed_mod: f32	= (keys.pressed(KeyCode::ShiftLeft) as u8) as f32;
	let mut camera_speed: f32	= 150.0;
	let zoom_speed: f32			= 0.5;

	// Arrow keys input used for changes to gravity.
	let left_right: f32	= (keys.pressed(KeyCode::Right) as i8 - keys.pressed(KeyCode::Left) as i8) as f32;
	let up_down: f32	= (keys.pressed(KeyCode::Up) as i8 - keys.pressed(KeyCode::Down) as i8) as f32;

	// Rotate/scale gravity when we press the arrow keys.
	change_gravity(constraints.as_mut(), up_down, left_right);

	// Extract the camera from our Query<> and control it.
	let camera_query = &mut mut_cameras.single_mut();
	let mut camera = (camera_query.0.as_mut(), camera_query.1.as_mut());
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
		&mut ui_state.zoom_slider
	);

	// Handle tool usage.
	if left_mouse_pressed {

		let cursor_position: Vec2	= get_cursor_position(&windows, &cameras);
        ev_tool_use.send(UseToolEvent::new(ui_state.selected_tool, cursor_position, Some(MouseButton::Left)));

		// let _ = grid.set_grid_cell_type(
		// 	cell_coordinates.x as usize,
		// 	cell_coordinates.y as usize,
		// 	SimGridCellType::Solid
		// );

		// Delete all particles in the cell we are turning into a solid.
		// let lookup_index: usize = grid.get_lookup_index(cell_coordinates);
		// grid.delete_all_particles_in_cell(
		// 	&mut commands,
		// 	constraints.as_mut(),
		// 	&particles,
		// 	lookup_index
		// );

	} else if right_mouse_pressed {

		let cursor_position: Vec2	= get_cursor_position(&windows, &cameras);
        ev_tool_use.send(UseToolEvent::new(ui_state.selected_tool, cursor_position, Some(MouseButton::Right)))

		// let _ = grid.set_grid_cell_type(
		// 	cell_coordinates.x as usize,
		// 	cell_coordinates.y as usize,
		// 	SimGridCellType::Air
		// );
	}
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