use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::events::*;
use crate::util::*;
use crate::ui::UIStateManager;
use crate::simulation::SimConstraints;

/// Debugging state controller.
pub fn handle_input(
    mut constraints:    ResMut<SimConstraints>,
	keys:				Res<Input<KeyCode>>,
	mouse:				Res<Input<MouseButton>>,
	mut mouse_motion:	EventReader<MouseMotion>,
	windows:			Query<&Window>,
	cameras:			Query<(&Camera, &GlobalTransform)>,
    mut ev_reset:           EventWriter<ResetEvent>,
    mut ev_tool_use:        EventWriter<UseToolEvent>,
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

	let gravity_rotation: i8	= keys.pressed(KeyCode::Right) as i8 -
									keys.pressed(KeyCode::Left) as i8;
	let gravity_magnitude: i8	= keys.pressed(KeyCode::Up) as i8 -
									keys.pressed(KeyCode::Down) as i8;
	let mut polar_gravity: Vec2	= cartesian_to_polar(constraints.gravity);
	polar_gravity.x				+= 200.0 * gravity_magnitude as f32 * constraints.timestep;
	polar_gravity.y				+= 4.0 * gravity_rotation as f32 * constraints.timestep;

	// Limit the magnitude of the vector to prevent ugly behavior near 0.0.
	polar_gravity.x				= f32::max(0.0, polar_gravity.x);
	constraints.gravity			= polar_to_cartesian(polar_gravity);

	let left_mouse_pressed: bool = mouse.pressed(MouseButton::Left);
	let right_mouse_pressed: bool = mouse.pressed(MouseButton::Right);

	// Get the mouse's motion between this and the last frame.
	let mut cursor_delta: Vec2 = Vec2::ZERO;
	for event in mouse_motion.read() {
		cursor_delta.x = event.delta.x;
		cursor_delta.y = event.delta.y;
	}

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
