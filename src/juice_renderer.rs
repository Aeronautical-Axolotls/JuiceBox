use bevy::{
	prelude::*,
	core_pipeline::prelude::ClearColor,
};
use crate::{util, simulation::sim_state_manager::SimParticle};

pub struct JuiceRenderer;
impl Plugin for JuiceRenderer {

	fn build(&self, app: &mut App) {
		app.insert_resource(ClearColor(util::JUICE_BLUE));

		app.add_systems(Startup, setup_renderer);
		
		app.add_systems(Update, update_particle_position);
		app.add_systems(Update, update_particle_color);
		app.add_systems(Update, update_particle_size);
	}
}

/// Custom rendering pipeline initialization.
fn setup_renderer(mut commands: Commands) {

	// Spawn a camera to view our simulation world!
	commands.spawn(Camera2dBundle::default());
}

/** Creates and links a new sprite to the specified particle; **Must be called each time a new 
	particle is added to the simulation!** */
pub fn link_particle_sprite(mut commands: &mut Commands, particle: Entity) {
	commands.entity(particle).insert(SpriteBundle::default());
}

/// Update the visual transform of all particles to be rendered.
fn update_particle_position(mut particles: Query<(&SimParticle, &mut Transform)>) {
	
	for (particle, mut transform) in particles.iter_mut() {
		transform.translation = Vec3 {
			x: particle.position.x,
			y: particle.position.y,
			/* IMPORTANT: Keep this at the same z-value for all particles.  This allows Bevy to do 
				sprite batching, cutting render costs by quite a bit.  If we change the z-index we 
				will likely see a large performance drop. */
			z: 0.0,
		};
	}
}

/// Update the color of all particles to be rendered.
fn update_particle_color(mut particles: Query<(&SimParticle, &mut Sprite)>) {
	
	for (particle, mut sprite) in particles.iter_mut() {
		let color: Color = util::generate_color_from_gradient(
			util::JUICE_GREEN, 
			util::JUICE_YELLOW, 
			util::JUICE_RED, 
			util::vector_magnitude(particle.velocity)
		);
		
		sprite.color = color;
	}
}

/// Update the size of all particles to be rendered.
fn update_particle_size(mut particles: Query<(&SimParticle, &mut Sprite)>) {
	
	for (_, mut sprite) in particles.iter_mut() {
		let size: f32 = 10.0;
		sprite.custom_size = Some(Vec2::splat(size));
	}
}