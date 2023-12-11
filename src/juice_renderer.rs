use bevy::{
	prelude::*,
	core_pipeline::prelude::ClearColor,
	sprite::{MaterialMesh2dBundle, Material2d},
	render::RenderApp,
};
use crate::{util, simulation::sim_state_manager::SimParticles};

pub struct JuiceRenderer;
impl Plugin for JuiceRenderer {

	fn build(&self, app: &mut App) {
		app.insert_resource(ClearColor(util::JUICE_BLUE));

		app.add_systems(Startup, setup_renderer);
		app.add_systems(Update, update_particle_render_data);
		
		// let mut render_app = app.sub_app_mut(RenderApp);
		// TODO: Add custom pipeline features here.
	}
}

/// Custom rendering pipeline initialization.
fn setup_renderer(
	mut commands:	Commands,
	mut meshes:		ResMut<Assets<Mesh>>,
	mut materials:	ResMut<Assets<ColorMaterial>>,
	mut particles:	ResMut<SimParticles>) {

	// Spawn a camera to view our simulation world!
	commands.spawn(Camera2dBundle::default());

	// Spawn test particle using a color gradient for material creation.
	let particle_color_material: ColorMaterial = ColorMaterial::from(
		util::generate_color_from_gradient(
			util::JUICE_GREEN, 
			util::JUICE_YELLOW, 
			util::JUICE_RED, 
			0.85
		)
	);
	commands.spawn(MaterialMesh2dBundle {
		mesh:		meshes.add(shape::Circle::new(10.0).into()).into(),
		material:	materials.add(particle_color_material),
		transform:	Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
		..default()
	});
}

/* Copy the position and velocity of each particle we need to draw via a Query to the ECS world.  
	Directly copy the particle position, and translate the velocity into a color. */
fn update_particle_render_data(
	mut particles:			ResMut<SimParticles>, 
	mut render_particles:	Query<(&mut Transform, &mut Handle<ColorMaterial>, Without<Camera2d>)>,
	mut materials:			ResMut<Assets<ColorMaterial>>) {
	
	// We don't want to crash, now do we?
	if particles.particle_position.len() < 1 {
		return;
	}
	
	for i in (0..particles.particle_count) {
		
	}
	
	/* For each particle that we have created to model a particle within our simulation, update 
		its position on the screen and its ColorMaterial. */
	for mut render_particle in render_particles.iter_mut() {
		render_particle.0.translation.x += 1.0;
		
		// If the materials haven't loaded yet, then Bevy will bypass this.
		if let Some(mut material) = materials.get_mut(render_particle.1.as_ref()) {
			material.color = util::generate_color_from_gradient(
				util::JUICE_GREEN, 
				util::JUICE_YELLOW, 
				util::JUICE_RED, 
				util::vector_magnitude(particles.particle_velocity[0])
			);
		}
	}
}