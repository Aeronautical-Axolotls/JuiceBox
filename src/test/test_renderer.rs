use bevy::prelude::*;
use crate::{juice_renderer::draw_vector_arrow, simulation::{SimConstraints, SimGrid}, util::cartesian_to_polar};

pub fn test_draw_vector_arrow(time: Res<Time>, gizmos: &mut Gizmos) {
	let dir: f32 = time.elapsed().as_secs_f32() * 16.0;
	let mag: f32 = (time.elapsed().as_secs_f32().sin() + 1.1) * 100.0;
	draw_vector_arrow(Vec2::ZERO, dir, mag, Color::PINK, gizmos);
}

pub fn test_draw_gravity_vector_arrow(
	constraints:	Res<SimConstraints>,
	grid:			Res<SimGrid>,
	mut gizmos:		Gizmos) {

	let polar_gravity: Vec2	= cartesian_to_polar(constraints.gravity);
	let arrow_base: Vec2	= Vec2 {
		x: (grid.dimensions.1 * grid.cell_size) as f32 / 2.0,
		y: (grid.dimensions.0 * grid.cell_size) as f32 / 2.0
	};

	draw_vector_arrow(arrow_base, polar_gravity.y, polar_gravity.x / 6.0, Color::GOLD, &mut gizmos);
}
