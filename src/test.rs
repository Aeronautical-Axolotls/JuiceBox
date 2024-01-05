use bevy::{
	prelude::*,
};
use crate::simulation::sim_state_manager::{
	SimGridCellType,
	SimGrid,
	add_particles_in_radius
};
use crate::juice_renderer::draw_vector_arrow;

pub fn construct_test_simulation_layout(grid: &mut SimGrid, mut commands: Commands)
{
	let grid_center: Vec2 = Vec2 {
		x: (grid.dimensions.1 * grid.cell_size) as f32 * 0.5,
		y: (grid.dimensions.0 * grid.cell_size) as f32 * 0.5,
	};
	let _test_particles = add_particles_in_radius(
		&mut commands,
		grid,
		3.5,
		100.0,
		Vec2 { x: grid_center[0], y: grid_center[1] },
		Vec2::ZERO
	);
	grid.cell_type[19][12] = SimGridCellType::Solid;
	grid.cell_type[20][12] = SimGridCellType::Solid;
	grid.cell_type[20][13] = SimGridCellType::Solid;
	grid.cell_type[20][14] = SimGridCellType::Solid;
	grid.cell_type[20][15] = SimGridCellType::Solid;
	grid.cell_type[19][15] = SimGridCellType::Solid;
}

pub fn test_draw_vector_arrow(time: Res<Time>, gizmos: &mut Gizmos) {
	let dir: f32 = time.elapsed().as_secs_f32() * 16.0;
	let mag: f32 = (time.elapsed().as_secs_f32().sin() + 1.1) * 100.0;
	draw_vector_arrow(Vec2::ZERO, dir, mag, Color::PINK, gizmos);
}