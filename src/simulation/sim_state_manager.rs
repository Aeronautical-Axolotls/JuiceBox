use std::f32::consts::PI;

use bevy::prelude::*;
use bevy::math::Vec2;
use crate::error::Error;
use crate::juice_renderer;

use super::*;

pub type Result<T> = core::result::Result<T, Error>;

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
pub fn add_particle(
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
pub fn delete_particle(
	commands:		&mut Commands,
	constraints:	&mut SimConstraints,
	particles:		&Query<(Entity, &mut SimParticle)>,
	grid:			&mut SimGrid,
	particle_id:	Entity) -> Result<()> {

	// Look for the particle in our particles query.
	if let Ok(particle) = particles.get(particle_id) {

		// Remove particle from lookup table and despawn it.
		grid.remove_particle_from_lookup(particle_id, particle.1.lookup_index);
		commands.entity(particle_id).despawn();

		/* BUG: This overflowed once while testing, and I'm betting it's because I misuse
			Entity::PLACEHOLDER.  Here is my silly little fix: */
		if constraints.particle_count > 0 {
			constraints.particle_count -= 1;
		}

		return Ok(());
	}

	Err(Error::InvalidEntityID("Invalid particle entity ID!"))
}

/// Reset all simulation components to their default state.
pub fn delete_all_particles(
	commands:		&mut Commands,
	constraints:	&mut SimConstraints,
	grid:			&mut SimGrid,
	particles:		&Query<(Entity, &mut SimParticle)>) {

	// KILL THEM ALL!!!
	for (particle_id, _) in particles.iter() {
		let _ = delete_particle(commands, constraints, particles, grid, particle_id);
	}
}

/** Returns a vector of entity ID's of each particle within a circle centered at `position` with
	radius `radius`; returns an empty vector if no particles are found. */
pub fn select_particles<'a>(
	particles:	&Query<(Entity, &mut SimParticle)>,
	grid:		&SimGrid,
	position:	Vec2,
	radius:		f32) -> Vec<Entity> {

	let mut selected_particles: Vec<Entity>	= Vec::new();

	// TODO: Maybe use map() here?  Idk.  Garrett I need u to explain map() to me I don't get it :(
	let selected_cell_coordinates: Vec<Vec2> = grid.select_grid_cells(position, radius);

	for i in 0..selected_cell_coordinates.len() {

		let cell_lookup_index: usize = grid.get_lookup_index(selected_cell_coordinates[i]);
		for particle_id in grid.get_particles_in_lookup(cell_lookup_index).iter() {

            // Skip particles we can't find
            let Ok(particle_entity) = particles.get(*particle_id) else {
                continue;
            };

            let particle = particle_entity.1;

			// Avoid an unnecessary sqrt() here:
			let distance: f32 = Vec2::distance_squared(position, particle.position);

			// If we are within our radius, add the particle to the list and return it!
			if distance < (radius * radius) {
				selected_particles.push(*particle_id);
			}
		}
	}

	selected_particles
}

pub fn add_faucet(
	commands:			&mut Commands,
	grid:				&mut SimGrid,
    faucet_pos:         Vec2,
    surface_direction:  Option<SimSurfaceDirection>,
    faucet_diameter:    f32,
    faucet_flow:        Vec2,
    ) -> Result<()> {

	if faucet_pos[0] < 0.0 || faucet_pos[0] > (grid.dimensions.1 * grid.cell_size) as f32 {
		return Err(Error::OutOfGridBounds(
			"X-coordinate for particle creation is out of grid bounds!"
		));
	}
    if faucet_pos[1] < 0.0 || faucet_pos[1] > (grid.dimensions.0 * grid.cell_size) as f32 {
		return Err(Error::OutOfGridBounds(
			"Y-coordinate for particle creation is out of grid bounds!"
		));
	}

    commands.spawn(
        SimFaucet::new(faucet_pos, surface_direction, faucet_diameter, faucet_flow)
    );


    Ok(())
}

pub fn add_drain(
	commands:			&mut Commands,
	grid:				&mut SimGrid,
    drain_pos:         Vec2,
    surface_direction:  Option<SimSurfaceDirection>
    ) -> Result<()> {

	if drain_pos[0] < 0.0 || drain_pos[0] > (grid.dimensions.1 * grid.cell_size) as f32 {
		return Err(Error::OutOfGridBounds(
			"X-coordinate for particle creation is out of grid bounds!"
		));
	}
    if drain_pos[1] < 0.0 || drain_pos[1] > (grid.dimensions.0 * grid.cell_size) as f32 {
		return Err(Error::OutOfGridBounds(
			"Y-coordinate for particle creation is out of grid bounds!"
		));
	}

    commands.spawn(
        SimDrain::new(drain_pos, surface_direction)
    );


    Ok(())
}

pub fn activate_components(
    commands:		&mut Commands,
    constraints:	&mut SimConstraints,
    particles:      &Query<(Entity, &mut SimParticle)>,
    faucets:        &Query<(Entity, &SimFaucet)>,
    drains:         &Query<(Entity, &SimDrain)>,
    grid:           &mut SimGrid,
    ) -> Result<()> {

    for (_, faucet) in faucets.iter() {
        faucet.run(commands, constraints, grid)?;
    }

    for (_, drain) in drains.iter() {
        drain.drain(commands, constraints, grid, particles)?;
    }

    Ok(())
}
