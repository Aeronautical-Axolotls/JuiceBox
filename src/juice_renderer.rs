use bevy::{
	prelude::*,
	core_pipeline::prelude::ClearColor,
	sprite::MaterialMesh2dBundle,
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
		
		// let mut render_app = app.sub_app_mut(RenderApp);
		// TODO: Add custom pipeline features here.
	}
}

/// Custom rendering pipeline initialization.
fn setup_renderer(
	mut commands:	Commands,
	mut meshes:		ResMut<Assets<Mesh>>,
	mut materials:	ResMut<Assets<ColorMaterial>>,
	mut particles:	Query<&mut SimParticle>) {

	// Spawn a camera to view our simulation world!
	commands.spawn(Camera2dBundle::default());
}

/// Creates and links the correct sprite to the specified particle.
pub fn link_particle_sprite(mut commands: Commands, particle: Entity) {
	commands.entity(particle).insert(SpriteBundle::default());
}

/// Update the position of all particles.
fn update_particle_position(mut particles: Query<(&SimParticle, &Sprite, &mut Transform)>) {
	
	for (particle, sprite, mut transform) in particles.iter_mut() {
		transform.translation = Vec3 {
			x: particle.position.x,
			y: particle.position.y,
			z: 0.0,
		};
	}
}

/// Update the color of all particles.
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

/// Update the size of all particles.
fn update_particle_size(mut particles: Query<(&SimParticle, &mut Sprite)>) {
	
	for (_, mut sprite) in particles.iter_mut() {
		let size: f32 = 10.0;
		sprite.custom_size = Some(Vec2::splat(size));
	}
}