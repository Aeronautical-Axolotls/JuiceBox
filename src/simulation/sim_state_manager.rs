use std::f32::consts::{PI, FRAC_2_PI, FRAC_PI_2, E, LOG2_E};

use bevy::prelude::*;
use bevy::math::Vec2;
use crate::error::Error;
use crate::juice_renderer;

use super::*;

pub type Result<T> = core::result::Result<T, Error>;

// pub struct SimStateManager;
// impl Plugin for SimStateManager {

// 	fn build(&self, app: &mut App) {
// 		app.insert_resource(SimConstraints::default());
// 		app.insert_resource(SimGrid::default());

// 		app.add_systems(Startup, setup);
// 		app.add_systems(Update, update);
// 	}
// }

// /// Simulation state manager initialization.
// fn setup(
// 	mut commands:		Commands,
// 	mut constraints:	ResMut<SimConstraints>,
// 	mut grid:			ResMut<SimGrid>) {

// 	test::construct_test_simulation_layout(grid.as_mut(), commands);
// 	// TODO: Get saved simulation data from most recently open file OR default file.
// 	// TODO: Population constraints, grid, and particles with loaded data.
// }

// /// Simulation state manager update; handles user interactions with the simulation.
// fn update(
// 	mut constraints:	ResMut<SimConstraints>,
// 	mut grid:			ResMut<SimGrid>) {

// 	// TODO: Check for and handle simulation saving/loading.
// 	// TODO: Check for and handle simulation pause/timestep change.
// 	// TODO: Check for and handle changes to simulation grid.
// 	make_grid_velocities_incompressible(grid.as_mut(), constraints.as_ref());
// 	// TODO: Check for and handle changes to gravity.
// 	// TODO: Check for and handle tool usage.
// }

/** Add many particles into the simulation within a radius.  Note that particle_density is
	the number of particles per unit radius. */
pub fn add_particles_in_radius(
	commands:			&mut Commands,
	constraints:		&mut SimConstraints,
	grid:				&mut SimGrid,
	particle_density:	f32,
	radius:				f32,
	center_position:	Vec2,
	velocity:			Vec2) {

	// Create center particle.
	let _center_particle = add_particle(commands, constraints, grid, center_position, velocity);

	// Density for the rings inside the circle.
	let ring_density: f32		= particle_density * 2.0;

	// Create concentric rings of particles that evenly space themselves out to form a circle!
	let ring_count: usize = 1 + (radius * ring_density / 20.0) as usize;
	for ring_index in 1..ring_count {

		/* Create each particle around the current ring. */
		let ring_radius: f32		= ring_index as f32 / ring_density * 10.0;
		let particle_count: usize	= (ring_radius as f32 * particle_density) as usize;
		for particle_index in 0..particle_count as usize {

			// Find the angle around the circle so we can correctly position this particle.
			let angle: f32 = particle_index as f32 * ((2.0 * PI) / particle_count as f32);

			// Find the position of the particle at the desired position around the ring.
			let particle_position: Vec2 = Vec2 {
				x: center_position[0] + (f32::cos(angle) * ring_radius),
				y: center_position[1] + (f32::sin(angle) * ring_radius),
			};
//
			// If particle_position is outside the grid bounds, this will not create a particle:
			let _particle = add_particle(commands, constraints, grid, particle_position, velocity);
		}
	}
}

/// Add particles into the simulation.
fn add_particle(
	commands:		&mut Commands,
	constraints:	&mut SimConstraints,
	grid:			&mut SimGrid,
	position:		Vec2,
	velocity:		Vec2) -> Result<()> {

	// Don't allow the user to create particles out of the simulation grid's bounds!
	if position[0] < 0.0 || position[0] > (grid.dimensions.1 * grid.cell_size) as f32 {
		return Err(Error::OutOfGridBounds(
			"X-coordinate for particle creation is out of grid bounds!"
		));
	}
	if position[1] < 0.0 || position[1] > (grid.dimensions.0 * grid.cell_size) as f32 {
		return Err(Error::OutOfGridBounds(
			"Y-coordinate for particle creation is out of grid bounds!"
		));
	}
	// If the cell we are inside of is a solid, don't create the particle!
	let cell_coordinates: Vec2 = grid.get_cell_coordinates_from_position(&position);
	if matches!(
		grid.cell_type[cell_coordinates[0] as usize][cell_coordinates[1] as usize],
		SimGridCellType::Solid) {
		return Err(Error::InvalidCellParticleCreation("Chosen cell is solid!"));
	}
	
	// Add every particle to the 0-cell's lookup at first; we will sort this next frame.
	let lookup_index: usize	= 0;
	let particle: Entity	= commands.spawn(
		SimParticle {
			position:		position,
			velocity:		velocity,
			lookup_index:	lookup_index,
		}
	).id();
	grid.add_particle_to_lookup(particle, lookup_index);
	
	constraints.particle_count += 1;
	
	// IMPORTANT: Links a sprite to each particle for rendering.
	juice_renderer::link_particle_sprite(commands, particle);

	Ok(())
}

/// Remove a particle with ID particle_id from the simulation.
fn delete_particle(
	mut commands:		&mut Commands,
	mut constraints:	&mut SimConstraints,
	mut particles:		Query<(Entity, &mut SimParticle)>,
	grid:				&mut SimGrid,
	particle_id:		Entity) -> Result<()> {
	
	// Look for the particle in our particles query.
	if let Ok(particle) = particles.get(particle_id) {
		
		// Remove particle from lookup table and despawn it.
		grid.remove_particle_from_lookup(particle_id, particle.1.lookup_index);
		commands.entity(particle_id).despawn();
		constraints.particle_count -= 1;
		
		return Ok(());
	}
	
	Err(Error::InvalidEntityID("Invalid particle entity ID!"))
}

/** Returns a vector of ID's of the particles within a circle centered at "position" with radius
	"radius." */
pub fn select_particles(
	particles:	Query<(Entity, &mut SimParticle)>,
	position:	Vec2,
	radius:		u32) -> Result<Vec<Entity>> {

	/* TODO: Rework this function to use a spatial lookup based on SimGrid.  If a particle is
		outside of the nearest grid cells, then skip checking it.  We can accomplish this in a
		parallel-friendly way by sorting a list of spatial lookups for particles based on the grid,
		then choosing the nearest 1/9/25/49 grid cells (based on radius). */

	let mut selected_particles: Vec<Entity> = Vec::new();

	for (entity_id, particle) in particles.iter() {
		let distance: f32 = position.distance(particle.position);
		if distance <= (radius as f32) {
			selected_particles.push(entity_id);
		}
	}

	if selected_particles.len() > 0 {
		Ok(selected_particles)
	} else {
		Err(Error::NoParticlesFound("cannot select any particles!"))
	}
}
