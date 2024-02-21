use bevy::{ecs::{system::SystemState, world}, prelude::*, transform::commands};
use crate::{juice_renderer::draw_vector_arrow, simulation::{sim_state_manager::add_particle, SimConstraints, SimGrid, SimParticle}, util::cartesian_to_polar};

/// Check to see if sprites are linked to particles.
#[test]
fn test_sprite_particle_linking() {

	// Create a testing Bevy application and world.
    let mut juicebox_test = App::new();
    juicebox_test.insert_resource(SimGrid::default());
    juicebox_test.insert_resource(SimConstraints::default());

	/* Once we queue commands and apply them to system state, we should have a particle with a
		sprite bound to it! */
	let mut system_state	= SystemState::<Commands>::new(&mut juicebox_test.world);
	let mut commands		= system_state.get_mut(&mut juicebox_test.world);

	/* Spawn a particle and link a sprite to it; We do not need the lookup index stuff so we
		simplify the test to the only two components we care about: the particle and the sprite. */
	let particle: Entity	= commands.spawn(
		SimParticle {
			position:		Vec2 { x: 66.098, y: 19.5 },
			velocity:		Vec2::ZERO,
			lookup_index:	0,
		}
	).id();
	commands.entity(particle).insert(SpriteBundle::default());

	// Apply commands to the world's system state.
	system_state.apply(&mut juicebox_test.world);

	// Count the number of particles w/ sprites bound; if we have one, we are good!
	let sprite_count: usize = juicebox_test.world.query::<(&SimParticle, With<Sprite>)>()
		.iter(&juicebox_test.world).len();

	assert_eq!(true, sprite_count == 1);
}

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
