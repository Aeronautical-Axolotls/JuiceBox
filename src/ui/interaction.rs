use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use crate::events::{GravityChangeEvent, ResetEvent, UseToolEvent};
use crate::util::*;
use crate::ui::UIStateManager;
use crate::simulation::SimConstraints;

/// Debugging state controller.
pub fn handle_input(
	keys:				Res<Input<KeyCode>>,
	mouse:				Res<Input<MouseButton>>,
	windows:			Query<&Window>,
	cameras:			Query<(&Camera, &GlobalTransform)>,
    mut ev_reset:       EventWriter<ResetEvent>,
    mut ev_tool_use:    EventWriter<UseToolEvent>,
	mut ev_gravity:		EventWriter<GravityChangeEvent>,
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
	ev_gravity.send(GravityChangeEvent::new(
		keys.pressed(KeyCode::Right) as i8 - keys.pressed(KeyCode::Left) as i8,
		keys.pressed(KeyCode::Up) as i8 - keys.pressed(KeyCode::Down) as i8
	));

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
