use bevy::prelude::*;
use crate::simulation::SimConstraints;
use crate::simulation::{
	SimGridCellType,
	SimGrid,
	sim_state_manager::add_particles_in_radius
};
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

	// Spawn a group of 3,147 particles at the center of the screen.
	let grid_center: Vec2 = Vec2 {
		x: (grid.dimensions.1 * grid.cell_size) as f32 * 0.5,
		y: (grid.dimensions.0 * grid.cell_size) as f32 * 0.5,
	};
	let _test_particles = add_particles_in_radius(
		&mut commands,
		constraints,
		grid,
		3.5,
		100.0,
		Vec2 { x: grid_center[0], y: grid_center[1] },
		Vec2::ZERO
	);

	/*// Spawn more particles to test spawning inside solids is rejected.
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