use bevy::{
	prelude::*,
	core_pipeline::prelude::ClearColor,
	sprite::MaterialMesh2dBundle,
};
use crate::{util, simulation::sim_state_manager::SimParticles};

pub struct JuiceRenderer;
impl Plugin for JuiceRenderer {

	fn build(&self, app: &mut App) {
		app.insert_resource(ClearColor(util::JUICE_BLUE));

		app.add_systems(Startup, setup_renderer);
		app.add_systems(Update, sync_particle_render_instances);
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
	
	for _ in 0..2 {
		commands.spawn(MaterialMesh2dBundle {
			mesh:		meshes.add(shape::Circle::new(10.0).into()).into(),
			material:	materials.add(particle_color_material.clone()),
			transform:	Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
			..default()
		});
	}
}

// Ensure the renderer is attempting to draw the correct number of particles.
fn sync_particle_render_instances(
	mut commands:			Commands,
	mut particles:			ResMut<SimParticles>, 
	mut render_particles:	Query<(Entity, With<Transform>, With<Handle<ColorMaterial>>)>,
	mut meshes:				ResMut<Assets<Mesh>>,
	mut materials:			ResMut<Assets<ColorMaterial>>) {
	
	// We don't want to crash, now do we?
	if particles.particle_count < 1 {
		return;
	}
	
	let particle_count: usize			= particles.particle_count;
	let render_particle_count: usize	= render_particles.iter().count();
	
	// If we have the correct number of render particles.
	if particle_count == render_particle_count {
		return;
		
		// If we need to remove render particles.
	} else if particle_count < render_particle_count {
		// BUG/TODO: This removes all particles when we go over the correct number.  Fix that.
		let particles_to_delete_count: usize	= render_particle_count - particle_count;
		let mut i: usize						= 0;
		
		for (render_particle, _, _) in render_particles.iter() {
			commands.entity(render_particle).despawn();
			
			// If we have deleted the correct number of particles.
			if i == particles_to_delete_count {
				return;
			}
			i += 1;
		}
		
		// If we need more render particles.
	} else {
		for i in 0..(particle_count - render_particle_count) {
			commands.spawn(MaterialMesh2dBundle {
				mesh:		meshes.add(shape::Circle::new(10.0).into()).into(),
				material:	materials.add(ColorMaterial::from(Color::WHITE)),
				transform:	Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
				..default()
			});
		}
	}
}

/* Copy the position and velocity of each particle we need to draw via a Query to the ECS world.  
	Directly copy the particle position, and translate the velocity into a color. */
fn update_particle_render_data(
	mut commands:			Commands,
	mut particles:			ResMut<SimParticles>, 
	mut render_particles:	Query<(&mut Transform, &mut Handle<ColorMaterial>)>,
	mut materials:			ResMut<Assets<ColorMaterial>>) {
	
	// We don't want to crash, now do we?
	if particles.particle_count < 1 || render_particles.iter().count() != particles.particle_count {
		return;
	}
	
	/* For each particle that we have created to model a particle within our simulation, update 
		its position on the screen and its ColorMaterial. */
	let mut i = 0;
	for mut render_particle in render_particles.iter_mut() {
		// Copy translation.
		render_particle.0.translation = Vec3 {
			x: particles.particle_position[i].x,
			y: particles.particle_position[i].y,
			z: 0.0
		};
		
		/* If the materials haven't loaded yet, then Bevy will bypass this.  Otherwise, create 
			the appropriate ColorMaterial. */
		if let Some(mut material) = materials.get_mut(render_particle.1.as_ref()) {
			material.color = util::generate_color_from_gradient(
				util::JUICE_GREEN, 
				util::JUICE_YELLOW, 
				util::JUICE_RED, 
				util::vector_magnitude(particles.particle_velocity[i])
			);
		}
		
		i += 1;
	}
}