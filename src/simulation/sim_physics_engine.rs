use bevy::prelude::*;
use crate::error::Error;
use super::{SimGrid, SimParticle, SimConstraints, SimGridCellType};
use super::util::*;

pub type Result<T> = core::result::Result<T, Error>;

/// Applies Particle velocities to grid velocity points
pub fn particles_to_grid(grid: &mut SimGrid, particles: &mut Query<(Entity, &mut SimParticle)>) -> SimGrid {

    // for velocity_u points and velocity_v points,
    // add up all particle velocities nearby scaled
    // by their distance / cell width (their influence)
    // then divide by the summation of all their
    // influences

    // This function, after applying particle velocities
    // to the grid, returns a copy of the grid, but the
    // values are all "change in" values

    // easy measurement for half the cell size
    let half_cell = grid.cell_size as f32 / 2.0;

    // Create new, blank grids
	let mut velocity_u = vec![vec![0.0; (grid.dimensions.0 + 1) as usize]; grid.dimensions.1 as usize];
    let mut velocity_v = vec![vec![0.0; grid.dimensions.0 as usize]; (grid.dimensions.1 + 1) as usize];

    // Go through each horizontal u velocity point in the MAC grid
    for row_index in 0..grid.velocity_u.len() {
        for col_index in 0..grid.velocity_u[row_index].len() {

            // Get (x, y) of current velocity point
            let pos = grid.get_velocity_point_pos(
                row_index,
                col_index,
                true);

            let left_center = pos - Vec2::new(half_cell, 0.0);
            let right_center = pos + Vec2::new(half_cell, 0.0);

            // If the velocity point lies on the simulation
            // boundary, skip it
            if left_center.x < 0.0 {
                continue;
            }

            if right_center.x > grid.dimensions.1 as f32 * grid.cell_size as f32 {
                continue;
            }

            // Determine if this velocity point lies between two air cells, and if so,
            // skip it
            let left_center_coords = grid.get_cell_coordinates_from_position(&left_center);
            let right_center_coords = grid.get_cell_coordinates_from_position(&right_center);

            if grid.cell_type[left_center_coords.x as usize][left_center_coords.y as usize] == SimGridCellType::Air && grid.cell_type[right_center_coords.x as usize][right_center_coords.y as usize] == SimGridCellType::Air {
                continue;
            }

            let mut scaled_velocity_sum = 0.0;

            let mut scaled_influence_sum = 0.0;

            particles.for_each(|(_, particle)| {
                let influence = find_influence(
                    particle.position,
                    pos,
                    grid.cell_size);

                if influence == 0.0 {
                    ()
                }

                scaled_influence_sum += influence;
                scaled_velocity_sum += particle.velocity[0] * influence;

            });

            if scaled_influence_sum == 0.0 {
                velocity_u[row_index][col_index] = 0.0;
                continue;
            }

            let new_velocity = scaled_velocity_sum / scaled_influence_sum;

            velocity_u[row_index][col_index] = new_velocity;
        }
    }

    // Do the same thing for vertical velocity points within the MAC grid
    for row_index in 0..grid.velocity_v.len() {
        for col_index in 0..grid.velocity_v[row_index].len() {

            let pos = grid.get_velocity_point_pos(
                row_index,
                col_index,
                false);

            let bottom_center = pos - Vec2::new(0.0, half_cell);
            let top_center = pos + Vec2::new(0.0, half_cell);

            if bottom_center.x < 0.0 {
                continue;
            }

            if top_center.x > grid.dimensions.0 as f32 * grid.cell_size as f32 {
                continue;
            }

            let bottom_center_coords = grid.get_cell_coordinates_from_position(&bottom_center);
            let top_center_coords = grid.get_cell_coordinates_from_position(&top_center);

            if grid.cell_type[bottom_center_coords.x as usize][bottom_center_coords.y as usize] == SimGridCellType::Air && grid.cell_type[top_center_coords.x as usize][top_center_coords.y as usize] == SimGridCellType::Air {
                continue;
            }

            let mut scaled_velocity_sum = 0.0;

            let mut scaled_influence_sum = 0.0;

            particles.for_each(|(_, particle)| {
                let influence = find_influence(
                    particle.position,
                    pos,
                    grid.cell_size);

                if influence == 0.0 {
                    ()
                }

                scaled_influence_sum += influence;
                scaled_velocity_sum += particle.velocity[1] * influence;
            });

            if scaled_influence_sum == 0.0 {
                velocity_v[row_index][col_index] = 0.0;
                continue;
            }

            let new_velocity = scaled_velocity_sum / scaled_influence_sum;

            velocity_v[row_index][col_index] = new_velocity;
        }
    }

    let old_grid = grid.clone();

    grid.velocity_u = velocity_u;
    grid.velocity_v = velocity_v;

    // Return the change grid
    create_change_grid(&old_grid, &grid)

}

/**
    Create a SimGrid with values containing the difference between
    The old grid and new grid
*/
fn create_change_grid(old_grid: &SimGrid, new_grid: &SimGrid) -> SimGrid {

    // Here we are creating a SimGrid that holds the delta or change
    // in values after applying the particle velocities to the grid.
    // These values are needed when interpolating the velocity
    // values transfered to the particles from the grid.

    let mut change_grid = old_grid.clone();
    let mut change_u  = old_grid.velocity_u.clone();
    let mut change_v = old_grid.velocity_v.clone();

    for row_index in 0..change_grid.velocity_u.len() {
        for col_index in 0..change_grid.velocity_u[row_index].len() {

            let mut change_in_u = new_grid.velocity_u[row_index][col_index] - old_grid.velocity_u[row_index][col_index];

            if change_in_u.is_nan() {
                change_in_u = 0.0;
            }

            change_u[row_index][col_index] = change_in_u;
        }
    }

    for row_index in 0..change_grid.velocity_v.len() {
        for col_index in 0..change_grid.velocity_v[row_index].len() {

            let mut change_in_v = new_grid.velocity_v[row_index][col_index] - old_grid.velocity_v[row_index][col_index];

            if change_in_v.is_nan() {
                change_in_v = 0.0;
            }

            change_v[row_index][col_index] = change_in_v;
        }
    }

    change_grid.velocity_u = change_u;
    change_grid.velocity_v = change_v;

    change_grid

}

/**
    Collects all the particles within a cell and returns
    a vector of particles with their ID and data
*/
fn collect_particles<'a>(
        grid: &SimGrid,
        center: Vec2,
        particles: &'a mut Query<(Entity, &mut SimParticle)>
    ) -> Vec<(Entity, Mut<'a, SimParticle>)> {

    let mut particle_bag = Vec::new();

    let index = get_lookup_index(center, grid.dimensions.0);

    let particle_ids = grid.get_particles_in_lookup(index);

    if particle_ids.len() == 0 {
        return Vec::new();
    }

    // Goes through all the particles and selects only
    // particles within the cell and adds them
    // to the bag
    particles.for_each_mut(|particle| {
        if particle_ids.contains(&particle.0) {
            particle_bag.push(particle);
        }
    });

    particle_bag

}

/**
    Interpolates new particle velocities from grid points for a given
    set of particles.
*/
fn apply_grid<'a>(
        particles: Vec<(Entity, Mut<'a, SimParticle>)>,
        grid: &SimGrid,
        change_grid: &SimGrid,
        pic_coef: f32,
    ) {

    // New velocity value using equation from section 7.6
    // in Fluid Simulation for Computer Graphics, Second Edition
    // (Bridson, Robert)

    for (_, mut particle) in particles {

        let interp_vel = interpolate_velocity(particle.position, &grid);
        let change_vel = interpolate_velocity(particle.position, &change_grid);

        let mut new_velocity = (pic_coef * interp_vel) + ((1.0 - pic_coef) * (particle.velocity + change_vel));

        if new_velocity.x.is_nan() || new_velocity.y.is_nan() {
            new_velocity = Vec2::ZERO;
        }

        particle.velocity = new_velocity;

    }

}

/// Apply grid velocities to particle velocities
pub fn grid_to_particles(
        grid: &mut SimGrid,
        change_grid: &SimGrid,
        particles: &mut Query<(Entity, &mut SimParticle)>,
        flip_pic_coef: f32
    ) {

    // Basic idea right now is to go through each cell,
    // figure out which particles are 'within' that cell,
    // then apply the grid transformation

    for row_index in 0..grid.dimensions.1 as usize {
        for col_index in 0..grid.dimensions.0 as usize {

            // Skip over looking for particles where
            // they are not located
            match grid.cell_type[row_index][col_index] {
                SimGridCellType::Air => {
                    continue;
                }
                SimGridCellType::Solid => {
                    continue;
                },
                SimGridCellType::Fluid => (),
            }

            // Grab the center postition of the cell
            let coords = Vec2::new(row_index as f32, col_index as f32);

            // Grab all the particles within this specific cell
            let particles_in_cell = collect_particles(grid, coords, particles);

            // if we find an old fluid cell without any particles in it
            // make it an air cell
            if particles_in_cell.len() == 0 {
                grid.cell_type[row_index][col_index] = SimGridCellType::Air;
                continue;
            }

            // Solve for the new velocities of the particles
            apply_grid(particles_in_cell, grid, change_grid, flip_pic_coef);
        }
    }
}

/// Update the particle's lookup_index based on position, then update the grid's lookup table.
pub fn update_particle_lookup(particle_id: Entity, particle: &mut SimParticle, grid: &mut SimGrid) {

	// Find the cell that this particle belongs to and update our spatial lookup accordingly.
	let cell_coordinates: Vec2	= grid.get_cell_coordinates_from_position(&particle.position);
	let lookup_index: usize		= get_lookup_index(cell_coordinates, grid.dimensions.0);

	// Remove the particle from its old lookup cell and place it here in its new one.
	if !grid.spatial_lookup[lookup_index].contains(&particle_id) {

		grid.remove_particle_from_lookup(particle_id, particle.lookup_index);
		grid.spatial_lookup[lookup_index].push(particle_id);
		particle.lookup_index = lookup_index;
	}
}

// Get a cell lookup index into our spatial lookup table.
pub fn get_lookup_index(cell_coordinates: Vec2, grid_row_count: u16) -> usize {
	(cell_coordinates[1] as u16 + (cell_coordinates[0] as u16 * grid_row_count)) as usize
}

/// Change each particle's position based on its velocity.
pub fn integrate_particles_and_update_spatial_lookup(
	constraints:	&SimConstraints,
	particles:		&mut Query<(Entity, &mut SimParticle)>,
	grid:			&mut SimGrid,
	delta_time:		f32) {

	for (id, mut particle) in particles.iter_mut() {
		// Change each particle's velocity by gravity.
		particle.velocity[0] += constraints.gravity[0] * delta_time;
		particle.velocity[1] += constraints.gravity[1] * delta_time;

		// Change each particle's position by velocity.
		particle.position[0] += particle.velocity[0] * delta_time;
		particle.position[1] += particle.velocity[1] * delta_time;

        // Update the grid cell this particle is in to be a fluid
        let coords = grid.get_cell_coordinates_from_position(&particle.position);
        grid.cell_type[coords.x as usize][coords.y as usize] = SimGridCellType::Fluid;

		// Update this particle's spatial lookup.
		update_particle_lookup(id, particle.as_mut(), grid);
	}
}

/// Handle particle collisions with walls.
pub fn handle_particle_collisions(
	constraints:	&SimConstraints,
	grid:			&SimGrid,
	particles:		&mut Query<(Entity, &mut SimParticle)>) {

	for (_, mut particle) in particles.iter_mut() {

		// TODO: Collide with solid cells.

		// Don't let particles escape the grid!
		let grid_width: f32		= (grid.cell_size * grid.dimensions.0) as f32;
		let grid_height: f32	= (grid.cell_size * grid.dimensions.1) as f32;
		if particle.position[0] < constraints.particle_radius {
			particle.position[0] = constraints.particle_radius;
			particle.velocity[0] = 0.0;
		}
		if particle.position[0] > grid_width {
			particle.position[0] = grid_width;
			particle.velocity[0] = 0.0;
		}
		if particle.position[1] < constraints.particle_radius {
			particle.position[1] = constraints.particle_radius;
			particle.velocity[1] = 0.0;
		}
		if particle.position[1] > grid_height {
			particle.position[1] = grid_height;
			particle.velocity[1] = 0.0;
		}
	}
}

/** Push particles apart so that we account for drift and grid cells with incorrect densities.
	TODO: Improve collision solving speed between particles within cells.  Lots of particles in
	one cell leads to a large slowdown. */
pub fn push_particles_apart(
	constraints:	&SimConstraints,
	grid:			&SimGrid,
	particles:		&mut Query<(Entity, &mut SimParticle)>,
	delta_time:		f32) {

	for i in 0..constraints.collision_iters_per_frame {

		// For each grid cell.
		for lookup_index in 0..grid.spatial_lookup.len() {

			// Create a vector of all particles in all of the surrounding cells.
			let nearby_particles: Vec<Entity>		= grid.get_nearby_particles(lookup_index);
			let possible_collisions: Vec<Entity>	= nearby_particles.clone();

			// For each particle within neighboring grid cell.
			for particle0_id in nearby_particles.iter() {

				// For each OTHER particle within this grid cell.
				for particle1_id in possible_collisions.iter() {

					// Don't process a collision between ourself!
					if particle0_id == particle1_id {
						continue;
					}

					// Get both particles involved in the collision.
					let particle_combo_result = particles.get_many_mut([
						*particle0_id,
						*particle1_id,
					]);
					let particle_combo = match particle_combo_result {
						Ok(particle_combo_result)	=> particle_combo_result,
						Err(_error)					=> {
							// eprintln!("Invalid particle combo; skipping!");
							continue;
						},
					};

					// Push both particles apart.
					separate_particle_pair(constraints, particle_combo, delta_time);
				}
			}
		}
	}
}

/// Helper function for push_particles_apart().
fn separate_particle_pair(
	constraints:		&SimConstraints,
	mut particle_combo:	[(Entity, Mut<'_, SimParticle>); 2],
	delta_time:			f32) {

	// Calculate a collision radius and distance to modify position (and break early if too far).
	let collision_multiplier: f32		= 2.0;
	let effective_radius: f32			= constraints.particle_radius * collision_multiplier;
	let collision_radius: f32			= effective_radius * 2.0;
	let collision_radius_squared: f32	= effective_radius * effective_radius;

	// Figure out if we even need to push the particles apart in the first place!
	let mut delta_x: f32	= particle_combo[0].1.position[0] - particle_combo[1].1.position[0];
	let mut delta_y: f32	= particle_combo[0].1.position[1] - particle_combo[1].1.position[1];
	let delta_squared: f32	= delta_x * delta_x + delta_y * delta_y;
	let delta: f32			= delta_squared.sqrt();
	if delta_squared > collision_radius_squared || delta_squared == 0.0 {
		return;
	}

	// Calculate the difference in position we need to separate the particles.
	let push_factor: f32	= 0.5;
	let delta_modifier: f32	= delta_time * push_factor * (collision_radius - delta) / delta;
	delta_x *= delta_modifier;
	delta_y *= delta_modifier;

	// Move the particles apart!
	particle_combo[0].1.position[0] += delta_x;
	particle_combo[0].1.position[1] += delta_y;
	particle_combo[1].1.position[0] -= delta_x;
	particle_combo[1].1.position[1] -= delta_y;

	particle_combo[0].1.velocity[0] += delta_x * collision_multiplier;
	particle_combo[0].1.velocity[1] += delta_y * collision_multiplier;
	particle_combo[1].1.velocity[0] -= delta_x * collision_multiplier;
	particle_combo[1].1.velocity[1] -= delta_y * collision_multiplier;
}

/** Force velocity incompressibility for each grid cell within the simulation.  Uses the
	Gauss-Seidel method. */
pub fn make_grid_velocities_incompressible(
	grid:			&mut SimGrid,
	constraints: 	&SimConstraints,
	delta_time:		f32) {
	
	// Allows the user to make the simulation go BRRRRRRR or brrr.
	for _ in 0..constraints.incomp_iters_per_frame {

		/* For each grid cell, calculate the inflow/outflow (divergence).  Then, find out how many
			surrounding cells are solid, then adjust grid velocities accordingly. */
		for row in 0..grid.dimensions.0 {
			for col in 0..grid.dimensions.1 {

				// Used to increase convergence time for our Gauss-Seidel implementation.
				let overrelaxation: f32	= 2.0;
				let stiffness: f32		= 1.0;
				let divergence: f32		= delta_time * calculate_cell_divergence(
					&grid,
					row as usize,
					col as usize,
					overrelaxation,
					stiffness
				);

				// Calculate and sum the solid modifier for each surrounding cell.
				let solids: [u8; 4]	= calculate_cell_solids(&grid, row as usize, col as usize);
				let solids_sum: u8	= solids.iter().sum();

				// Calculate solid modifier for each surrounding cell.
				let left_solid: f32		= solids[0] as f32 / solids_sum as f32;
				let right_solid: f32	= solids[1] as f32 / solids_sum as f32;
				let up_solid: f32		= solids[2] as f32 / solids_sum as f32;
				let down_solid: f32		= solids[3] as f32 / solids_sum as f32;

				// Force incompressibility on this cell.
				// BUG: These signs might be backwards...
				grid.velocity_u[row as usize][col as usize]			+= divergence * left_solid;
				grid.velocity_u[row as usize][(col + 1) as usize]	-= divergence * right_solid;
				grid.velocity_v[row as usize][col as usize]			-= divergence * up_solid;
				grid.velocity_v[(row + 1) as usize][col as usize]	+= divergence * down_solid;
			}
		}
	}
}

/** Calculate the divergence (inflow/outflow) of a grid cell.  If this number is not zero, then
	the fluid must be made incompressible.  **A negative divergence indicates there is too much
	inflow, whereas a positive divergence indicates too much outflow.**  Overrelaxation is used
	to increase the convergence of our divergence algorithm (ironic) dramatically.
	**Overrelaxation values must be between 1 and 2.** */
fn calculate_cell_divergence(
	grid:			&SimGrid,
	cell_row:		usize,
	cell_col:		usize,
	overrelaxation: f32,
	stiffness:		f32
) -> f32 {

	/* Retrieve velocities for each face of the current cell.  Note: this will not go out of
		bounds of the velocity arrays; each array is guaranteed to have sufficient space allocated
		to index like this. */
	let left_velocity: f32	= grid.velocity_u[cell_row][cell_col];
	let right_velocity: f32	= grid.velocity_u[cell_row][cell_col + 1];
	let up_velocity: f32	= grid.velocity_v[cell_row][cell_col];
	let down_velocity: f32	= grid.velocity_v[cell_row + 1][cell_col];

	// BUG: The up and down flows may need to be reversed.
	let x_divergence: f32	= right_velocity - left_velocity;
	let y_divergence: f32	= up_velocity - down_velocity;

	/* Calculate a stiffness factor to account for drift, where more dense regions need harder
		pushes to become less dense. */
	let target_density: f32		= 0.0;
	let density_actual: f32		= 0.0;
	let density_factor: f32		= stiffness * (density_actual - target_density);

	let divergence: f32		= overrelaxation * (x_divergence + y_divergence) - density_factor;

	// ==============================================================
	// TODO: Adjust divergence based on particle density in the cell.
	// ==============================================================

	divergence
}

/// Returns the cell solid modifiers (0 or 1) for cells in the order of: left, right, up, down.
fn calculate_cell_solids(grid: &SimGrid, cell_row: usize, cell_col: usize) -> [u8; 4] {

	/* Calculate collision modifiers for each cell face.  Note that we must perform a wrapping
		subtraction to prevent an underflow for our usize types. */
	let collision_left: u8	= grid.get_cell_type_value(usize::wrapping_sub(cell_col, 1), cell_col);
	let collision_right: u8	= grid.get_cell_type_value(cell_row, cell_col + 1);
	let collision_up: u8	= grid.get_cell_type_value(cell_row, usize::wrapping_sub(cell_row, 1));
	let collision_down: u8	= grid.get_cell_type_value(cell_row + 1, cell_col);

	[collision_left, collision_right, collision_up, collision_down]
}
