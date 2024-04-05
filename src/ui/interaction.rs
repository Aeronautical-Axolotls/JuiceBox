use std::borrow::BorrowMut;

use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::events::{ResetEvent, UseToolEvent};
use crate::util::*;
use crate::ui::UIStateManager;
use crate::simulation::{change_gravity, SimConstraints, SimGrid};

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
    mut ev_reset:       EventWriter<ResetEvent>,
    mut ev_tool_use:    EventWriter<UseToolEvent>,
    mut ui_state:       ResMut<UIStateManager>) {

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

	// Rotate/scale gravity when we press the arrow keys.
	let left_right: i8	= keys.pressed(KeyCode::Right) as i8 - keys.pressed(KeyCode::Left) as i8;
	let up_down: i8		= keys.pressed(KeyCode::Up) as i8 - keys.pressed(KeyCode::Down) as i8;
	change_gravity(constraints.as_mut(), up_down as f32, left_right as f32);

	// Camera controller input.
	let camera_horizontal_move: i8	= keys.pressed(KeyCode::D) as i8 - keys.pressed(KeyCode::A) as i8;
	let camera_vertical_move: i8	= keys.pressed(KeyCode::W) as i8 - keys.pressed(KeyCode::S) as i8;
	let camera_zoom_change: i8		= keys.pressed(KeyCode::E) as i8 - keys.pressed(KeyCode::Q) as i8;
	let camera_speed_mod: f32		= (keys.pressed(KeyCode::ShiftLeft) as u8) as f32;

	// Extract the camera from our Query<> and control it.
	let camera_query = &mut mut_cameras.single_mut();
	let mut camera = (camera_query.0.as_mut(), camera_query.1.as_mut());
	control_camera(
		&time,
		&grid,
		&mut constraints,
		&mut camera,
		150.0,
		0.5,
		camera_speed_mod,
		camera_horizontal_move,
		camera_vertical_move,
		camera_zoom_change
	);

	// Handle tool usage.
	let left_mouse_pressed: bool = mouse.pressed(MouseButton::Left);
	let right_mouse_pressed: bool = mouse.pressed(MouseButton::Right);
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
