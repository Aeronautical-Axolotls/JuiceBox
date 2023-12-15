use bevy::{
	prelude::*,
};

pub fn test_draw_vector_arrow(time: Res<Time>) {
	let dir: f32 = time.elapsed().as_secs_f32() * 16.0;
	let mag: f32 = (time.elapsed().as_secs_f32().sin() + 1.1) * 100.0;
	draw_vector_arrow(Vec2::ZERO, dir, mag, &mut gizmos);
}