use bevy::prelude::*;
use crate::juice_renderer::draw_selection_circle;
use crate::simulation::sim_state_manager::{
	select_particles,
	delete_particle,
};
use crate::simulation::{
	SimConstraints,
	SimParticle,
	SimGrid,
	sim_state_manager::add_particles_in_radius,
};
use crate::util::get_cursor_position;

/// Create a simulation layout for testing.
pub fn construct_test_simulation_layout(
	constraints:	&mut SimConstraints,
	grid:			&mut SimGrid,
	mut commands:	Commands) {
	
	// Create a bunch of solid cells.
	/*grid.cell_type[19][12] = SimGridCellType::Solid;
	grid.cell_type[20][12] = SimGridCellType::Solid;
	grid.cell_type[20][13] = SimGridCellType::Solid;
	grid.cell_type[20][14] = SimGridCellType::Solid;
	grid.cell_type[20][15] = SimGridCellType::Solid;
	grid.cell_type[19][15] = SimGridCellType::Solid;*/

	// Spawn a small test group of particles at the center of the screen.
	let grid_center: Vec2 = Vec2 {
		x: (grid.dimensions.1 * grid.cell_size) as f32 * 0.5,
		y: (grid.dimensions.0 * grid.cell_size) as f32 * 0.5,
	};
	
	let _test_particles = add_particles_in_radius(
		&mut commands,
		constraints,
		grid,
		2.0,
		25.0,
		Vec2 { x: grid_center[0], y: grid_center[1] },
		Vec2::ZERO
	);
	
	// Spawn a group of 3,147 particles at the center of the screen.
	/* let _test_particles = add_particles_in_radius(
		&mut commands,
		constraints,
		grid,
		3.5,
		100.0,
		Vec2 { x: grid_center[0], y: grid_center[1] },
		Vec2::ZERO
	);

	// Spawn more particles to test spawning inside solids is rejected.
	let _moar_test_particles = add_particles_in_radius(
		&mut commands,
		grid,
		2.35,
		50.0,
		Vec2 { x: 140.0, y: 45.0 },
		Vec2::ZERO
	);

	// Spawn even MOAR particles to test spawning inside solids is rejected.  ~~UwU~~
	let _moar_test_particles = add_particles_in_radius(
		&mut commands,
		grid,
		10.0,
		20.0,
		Vec2 { x: 100.0, y: 100.0 },
		Vec2::ZERO
	);*/
}

/// Test particle selection.
pub fn test_select_particles(
	commands:			&mut Commands,
	constraints:		&mut SimConstraints,
	mut grid:			&mut SimGrid,
	mut particles:		&Query<(Entity, &mut SimParticle)>,
	windows:			&Query<&Window>,
	cameras:			&Query<(&Camera, &GlobalTransform)>,
	mut gizmos:			&mut Gizmos) {
	
	let radius: f32				= 55.0;
	let cursor_position: Vec2	= get_cursor_position(windows, cameras);
	draw_selection_circle(gizmos, cursor_position, radius, Color::YELLOW);
	
	// Test particle selection.
	let selected_particles: Vec<Entity> = select_particles(
		particles,
		grid,
		cursor_position,
		radius
	);
	
	// Delete them to prove it worked!
	for particle in selected_particles.iter() {
		let _ = delete_particle(commands, constraints, particles, grid, *particle);
	}
}

/// Test grid cell selection.
pub fn test_select_grid_cells(
	commands:			&mut Commands,
	constraints:		&mut SimConstraints,
	mut grid:			&mut SimGrid,
	mut particles:		&Query<(Entity, &mut SimParticle)>,
	windows:			&Query<&Window>,
	cameras:			&Query<(&Camera, &GlobalTransform)>,
	mut gizmos:			&mut Gizmos) {
	
	let radius: f32				= 55.0;
	let cursor_position: Vec2	= get_cursor_position(windows, cameras);
	
	// Test cell selection.
	let selected_cells: Vec<Vec2> = grid.select_grid_cells(cursor_position, radius);
	for i in 0..selected_cells.len() {
		
		let half_cell_size: f32 = grid.cell_size as f32 * 0.5;
		let mut cell_position: Vec2 = grid.get_cell_position_from_coordinates(selected_cells[i]);
		cell_position.x += half_cell_size;
		cell_position.y += half_cell_size;
		
		gizmos.rect_2d(
			cell_position,
			0.0,
			Vec2 {
				x: grid.cell_size as f32,
				y: grid.cell_size as f32,
			},
			Color::BLACK
		);
	}
}