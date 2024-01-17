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

/** Returns a vector of entity ID's of each particle within a circle centered at `position` with 
	radius `radius`; returns an empty vector if no particles are found. */
pub fn select_particles<'a>(
	particles:	&Query<(Entity, &mut SimParticle)>,
	grid:		&SimGrid,
	position:	Vec2,
	radius:		f32) -> Vec<Entity> {
	
	let mut selected_particles: Vec<Entity>	= Vec::new();
	
	// TODO: Maybe use map() here?  Idk.  Garrett I need u to explain map() to me I don't get it :(
	let selected_cell_coordinates: Vec<Vec2> = select_grid_cells(grid, position, radius);
	
	for i in 0..selected_cell_coordinates.len() {
		
		let cell_lookup_index: usize = get_lookup_index(selected_cell_coordinates[i], grid.dimensions.0);
		for particle_id in grid.get_particles_in_lookup(cell_lookup_index).iter() {
			
			// TODO: Error checking here.  Don't use unwrap() in production!
			let particle: &SimParticle			= particles.get(*particle_id).unwrap().1;
			
			let radius_sqrd: f32				= radius * radius;
			let selection_position_sqrd: f32	= 
				position.x * position.x + 
				position.y * position.y;
			let particle_position_sqrd: f32		= 
				particle.position.x * particle.position.x + 
				particle.position.y * particle.position.y;
			
			// If we are within our radius, add the particle to the list and return it!
			//if selection_position_sqrd - particle_position_sqrd < radius_sqrd {
				selected_particles.push(*particle_id);
			//}
		}
	}
	
	selected_particles
}

/** Selects grid cells that entirely cover the a circle of radius `radius` centered at `position`; 
	returns a Vector containing each cell's coordinates. */
pub fn select_grid_cells(grid: &SimGrid, position: Vec2, radius: f32) -> Vec<Vec2> {
	
	/* If we are less than a cell in radius, the function will only search 1 cell.  That is 
		incorrect, as we could still need to search 4 cells if the selection is positioned 
		properly.  Therefore, we cap the radius for selection-cell bound checking to 2.5, but 
		leave the true radius untouched to retain proper particle selection behavior. */
	let min_selection_size: f32 = grid.cell_size as f32 / 2.0;
	let adj_radius: f32			= f32::max(min_selection_size, radius);
	
	/* Find our min/max world coordinates for cells to search.  Subtract cell size to account for 
		the selection area potentially not being perfectly centered; this will ensure we always 
		check the full possible number of cells our selection may be concerned with. We will check 
		one or two extra cells, but I believe consistent behavior is worth 4 extra cell checks. */
	let selection_max_bound: Vec2 = Vec2 {
		x: position.x + adj_radius + grid.cell_size as f32,
		y: position.y + adj_radius + grid.cell_size as f32,
	};
	let selection_min_bound: Vec2 = Vec2 {
		x: position.x - adj_radius,
		y: position.y - adj_radius,
	};
	
	// Find the number of cells we need to check.
	let mut x_cell_count: f32			= selection_max_bound.x - selection_min_bound.x;
	let mut y_cell_count: f32			= selection_max_bound.y - selection_min_bound.y;
	x_cell_count						/= grid.cell_size as f32;
	y_cell_count						/= grid.cell_size as f32;
	let cells_in_selection_count: usize	= (x_cell_count * y_cell_count) as usize;
	
	// Figure out which grid cells we are actually going to be checking.
	let mut cells_in_selection: Vec<Vec2>	= vec![Vec2::ZERO; cells_in_selection_count];
	for cell_index in 0..cells_in_selection_count {
		
		/* BUG: Sometimes the top two corner cells of the selection "flicker", and the sides have 
			an extra cell jutting out.  Not sure why, but my guess is it's a type casting or 
			rounding issue; not important (for now).  The corner flickering does affect the number 
			of cells checked, however the extra cell jutting out does not (making me think the 
			latter is a rendering issue).  Finally, the algorithm breaks down a little bit extra 
			if the radius is not a multiple of the grid cell size. */
		
		/* For each cell, get the grid coordinates from the selection's minimum bound.  Then, 
			move row-major to the right and up for each cell in our selection list. */
		let mut cell_coordinates: Vec2 = grid.get_cell_coordinates_from_position(
			&selection_min_bound
		);
		cell_coordinates.x -= f32::floor((cell_index as f32 / y_cell_count as f32) % y_cell_count as f32);
		cell_coordinates.y += f32::floor(cell_index as f32 % x_cell_count);
		
		cells_in_selection[cell_index] = cell_coordinates;
	}
	
	cells_in_selection
}