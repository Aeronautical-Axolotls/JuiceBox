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
    // to the grid, returns the previous grid

    // easy measurement for half the cell size
    let half_cell = grid.cell_size as f32 / 2.0;

    // Create new, blank grids
	let mut velocity_u = vec![vec![0.0; (grid.dimensions.0 + 1) as usize]; grid.dimensions.1 as usize];
    let mut velocity_v = vec![vec![0.0; grid.dimensions.0 as usize]; (grid.dimensions.1 + 1) as usize];

    // Go through each horizontal u velocity point in the MAC grid
    for row_index in 0..grid.dimensions.0 as usize {
        for col_index in 0..grid.dimensions.1 as usize + 1 {

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

                if influence != 0.0 {
                    scaled_influence_sum += influence;
                    scaled_velocity_sum += particle.velocity[0] * influence;
                }
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
    for row_index in 0..grid.dimensions.0 as usize + 1 {
        for col_index in 0..grid.dimensions.1 as usize {

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

                if influence != 0.0 {
                    scaled_influence_sum += influence;
                    scaled_velocity_sum += particle.velocity[1] * influence;
                }

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

    old_grid

}

/**
    Create a SimGrid with values containing the difference between
    The old grid and new grid
*/
pub fn create_change_grid(old_grid: &SimGrid, new_grid: &SimGrid) -> SimGrid {

    // Here we are creating a SimGrid that holds the delta or change
    // in values after applying the particle velocities to the grid.
    // These values are needed when interpolating the velocity
    // values transfered to the particles from the grid.

    let (rows, cols) = old_grid.dimensions;

    let mut change_grid = old_grid.clone();
	let mut change_u = vec![vec![0.0; (old_grid.dimensions.0 + 1) as usize]; old_grid.dimensions.1 as usize];
    let mut change_v = vec![vec![0.0; old_grid.dimensions.0 as usize]; (old_grid.dimensions.1 + 1) as usize];

    for row_index in 0..rows as usize {
        for col_index in 0..(cols as usize + 1) {

            let change_in_u = new_grid.velocity_u[row_index][col_index] - old_grid.velocity_u[row_index][col_index];

            change_u[row_index][col_index] = change_in_u;
        }
    }

    for row_index in 0..(rows as usize + 1) {
        for col_index in 0..cols as usize {

            let change_in_v =  new_grid.velocity_v[row_index][col_index] - old_grid.velocity_v[row_index][col_index];

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

    let index = grid.get_lookup_index(center);

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

        let pic_velocity = interp_vel;
        let flip_velocity =  particle.velocity + change_vel;
        let new_velocity = (pic_coef * pic_velocity) + ((1.0 - pic_coef) * flip_velocity);
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

            // Solve for the new velocities of the particles
            apply_grid(particles_in_cell, grid, change_grid, flip_pic_coef);
        }
    }
}

/// Update the particle's lookup_index based on position, then update the grid's lookup table.
pub fn update_particle_lookup(particle_id: Entity, particle: &mut SimParticle, grid: &mut SimGrid) {

	// Find the cell that this particle belongs to and update our spatial lookup accordingly.
	let cell_coordinates: Vec2	= grid.get_cell_coordinates_from_position(&particle.position);
	let lookup_index: usize		= grid.get_lookup_index(cell_coordinates);

	// Remove the particle from its old lookup cell and place it here in its new one.
	if !grid.spatial_lookup[lookup_index].contains(&particle_id) {

		grid.remove_particle_from_lookup(particle_id, particle.lookup_index);
		grid.spatial_lookup[lookup_index].push(particle_id);
		particle.lookup_index = lookup_index;
	}
}

/** For each particle: integrate velocity into position, update cell type, update spatial lookup,
	and update density value of the cell the particle is in. */
pub fn update_particles(
	constraints:	&SimConstraints,
	particles:		&mut Query<(Entity, &mut SimParticle)>,
	grid:			&mut SimGrid,
	delta_time:		f32) {

	grid.clear_density_values();

	for (id, mut particle) in particles.iter_mut() {
		
		// Integrate the particles while handling collisions.
		let target_velocity: Vec2 = particle.velocity + constraints.gravity * delta_time;
		let target_position: Vec2 = particle.position + target_velocity * delta_time;
		integrate_particle_with_collisions(
			constraints,
			grid,
			particle.as_mut(),
			&target_position,
			&target_velocity
		);

		// Update the grid's spatial lookup based on this particle's position!
		update_particle_lookup(id, particle.as_mut(), grid);

		// Update the grid's density value for this current cell.
		grid.update_grid_density(particle.position);
	}
}

/// Find the maximum distance a particle can move before hitting a solid!
fn integrate_particle_with_collisions(
	constraints:		&SimConstraints,
	grid:				&SimGrid,
	particle:			&mut SimParticle,
	target_position:	&Vec2,
	target_velocity:	&Vec2) {

	let target_coordinates: Vec2	= grid.get_cell_coordinates_from_position(&target_position);
	let target_cell_type: u8		= grid.get_cell_type_value(
		target_coordinates.x as usize,
		target_coordinates.y as usize
	);
	
	// If the target position is not inside of a solid cell, move as normal.
	if target_cell_type != 0 {
		particle.position = *target_position;
		particle.velocity = *target_velocity;
		
		// While we are headed for a solid cell, collide with it!
	} else {
		let cell_center: Vec2 = grid.get_cell_center_position_from_coordinates(&target_coordinates);
		
		// Check which direction the particle moved into the cell from this frame.
		let cell_half_size: f32	= (grid.cell_size as f32) / 2.0;
		let cell_left: f32		= cell_center.x - cell_half_size;// - constraints.particle_radius;
		let cell_right: f32		= cell_center.x + cell_half_size;// + constraints.particle_radius;
		let cell_top: f32		= cell_center.y + cell_half_size;// + constraints.particle_radius;
		let cell_bottom: f32	= cell_center.y - cell_half_size;// - constraints.particle_radius;
		
		let tolerance: f32		= 0.1;
		
		if particle.position.x <= cell_left && target_position.x >= cell_left {
			particle.position.x = cell_left - tolerance;
			particle.velocity.x = 0.0;
		} else if particle.position.x >= cell_right && target_position.x <= cell_right {
			particle.position.x = cell_right + tolerance;
			particle.velocity.x = 0.0;
		} else {
			particle.velocity.x = target_velocity.x;
			particle.position.x = target_position.x;
		}
		
		if particle.position.y <= cell_bottom && target_position.y >= cell_bottom {
			particle.position.y = cell_bottom - tolerance;
			particle.velocity.y = 0.0;
		} else if particle.position.y >= cell_top && target_position.y <= cell_top {
			particle.position.y = cell_top + tolerance;
			particle.velocity.y = 0.0;
		} else {
			particle.velocity.y = target_velocity.y;
			particle.position.y = target_position.y;
		}
	}
}

/// Handle particle collisions with the grid.
pub fn handle_particle_grid_collisions(
	constraints:	&SimConstraints,
	grid:			&SimGrid,
	particles:		&mut Query<(Entity, &mut SimParticle)>) {

	for (_, mut particle) in particles.iter_mut() {
		
		// Don't let particles escape the grid!
		let grid_width: f32		= (grid.cell_size * grid.dimensions.0) as f32;
		let grid_height: f32	= (grid.cell_size * grid.dimensions.1) as f32;
		
		// Left/right collision checks.
		if particle.position[0] < constraints.particle_radius {
			particle.position[0] = constraints.particle_radius;
			particle.velocity[0] = 0.0;
		} else if particle.position[0] > grid_width - constraints.particle_radius {
			particle.position[0] = grid_width - constraints.particle_radius;
			particle.velocity[0] = 0.0;
		}
		
		// Up/down collision checks.
		if particle.position[1] < constraints.particle_radius {
			particle.position[1] = constraints.particle_radius;
			particle.velocity[1] = 0.0;
		} else if particle.position[1] > grid_height - constraints.particle_radius {
			particle.position[1] = grid_height - constraints.particle_radius;
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
	particles:		&mut Query<(Entity, &mut SimParticle)>) {

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
					separate_particle_pair(constraints, grid, particle_combo);
				}
			}
		}
	}
}

/// Helper function for push_particles_apart().
fn separate_particle_pair(
	constraints:		&SimConstraints,
	grid:				&SimGrid,
	mut particle_combo:	[(Entity, Mut<'_, SimParticle>); 2]) {

	// Collision radii used to find the particle pair's push force on each other.
	let collision_radius: f32			= constraints.particle_radius * 2.0;
	let collision_radius_squared: f32	= collision_radius * collision_radius;

	// Figure out if we even need to push the particles apart in the first place!
	let mut delta_position: Vec2 = Vec2 {
		x: particle_combo[0].1.position[0] - particle_combo[1].1.position[0],
		y: particle_combo[0].1.position[1] - particle_combo[1].1.position[1]
	};
	let distance_squared: f32	= (delta_position.x * delta_position.x) + (delta_position.y * delta_position.y);
	if distance_squared > collision_radius_squared || distance_squared <= 0.01 {
		return;
	}

	// Calculate the distance we need to separate the particles by.
	let distance: f32			= distance_squared.sqrt();
	let separation_scale: f32	= 0.5 * (collision_radius - distance) / distance;
	delta_position *= separation_scale;

	// Move the particles apart!
	let target_velocity0: Vec2 = particle_combo[0].1.velocity;
	let target_velocity1: Vec2 = particle_combo[1].1.velocity;
	
	let target_position0: Vec2 = particle_combo[0].1.position + delta_position;
	let target_position1: Vec2 = particle_combo[1].1.position - delta_position;
	
	integrate_particle_with_collisions(
		constraints,
		grid,
		particle_combo[0].1.as_mut(),
		&target_position0,
		&target_velocity0
	);
	integrate_particle_with_collisions(
		constraints,
		grid,
		particle_combo[1].1.as_mut(),
		&target_position1,
		&target_velocity1
	);
}

/** Force velocity incompressibility for each grid cell within the simulation.  Uses the
	Gauss-Seidel method. */
pub fn make_grid_velocities_incompressible(
	grid:			&mut SimGrid,
	constraints: 	&mut SimConstraints) {

	// Get the "particle rest density" for the simulation domain.
	let mut fluid_cell_count: f32		= 0.0;
	let mut density_sum: f32			= 0.0;

	for i in 0..grid.density.len() {
		density_sum			+= grid.density[i];
		fluid_cell_count	+= 1.0;
	}

	if fluid_cell_count > 0.0 {
		constraints.particle_rest_density = density_sum / fluid_cell_count;
	}

	// Allows the user to make the simulation go BRRRRRRR or brrr.
	for _ in 0..constraints.incomp_iters_per_frame {

		/* For each grid cell, calculate the inflow/outflow (divergence).  Then, find out how many
			surrounding cells are solid, then adjust grid velocities accordingly. */
		for row in 0..grid.dimensions.0 {
			for col in 0..grid.dimensions.1 {

				// Continue if we are not inside of a fluid cell.
				if grid.cell_type[row as usize][col as usize] != SimGridCellType::Fluid {
					continue;
				}

				// Calculate and sum the solid modifier for each surrounding cell.
				let solids: [u8; 5]	= calculate_cell_solids(&grid, row as usize, col as usize);
				let left_solid: u8	= solids[1];
				let right_solid: u8	= solids[2];
				let up_solid: u8	= solids[3];
				let down_solid: u8	= solids[4];

				let solids_sum: u8	= left_solid + right_solid + up_solid + down_solid;
				if solids_sum == 0 {
					continue;
				}

				// Determine the inflow/outflow of the current cell.
				let mut divergence: f32 = calculate_cell_divergence(
					&grid,
					row as usize,
					col as usize
				);

				// Density calculations.
				if constraints.particle_rest_density > 0.0 {
					let stiffness: f32		= 1.0;
					let cell_coordinates: Vec2 = Vec2 {
						x: row as f32,
						y: col as f32
					};
					let density: f32		= grid.density[grid.get_lookup_index(cell_coordinates)];
					let compression: f32	= density - constraints.particle_rest_density;
					if compression > 0.0 {
						divergence -= stiffness * compression;
					}
				}

				// Force incompressibility on this cell.
				let overrelaxation: f32	= 1.9;
				let momentum: f32		= overrelaxation * ((0.0 - divergence) / solids_sum as f32);
				
				grid.velocity_u[row as usize][col as usize]			-= momentum * left_solid as f32;
				grid.velocity_u[row as usize][(col + 1) as usize]	+= momentum * right_solid as f32;
				grid.velocity_v[row as usize][col as usize]			+= momentum * up_solid as f32;
				grid.velocity_v[(row + 1) as usize][col as usize]	-= momentum * down_solid as f32;
			}
		}
	}
}

/** Calculate the divergence (inflow/outflow) of a grid cell.  If this number is not zero, then
	the fluid must be made incompressible.  **A negative divergence indicates there is too much
	inflow, whereas a positive divergence indicates too much outflow.** */
fn calculate_cell_divergence(
	grid:			&SimGrid,
	cell_row:		usize,
	cell_col:		usize,
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
	let divergence: f32		= x_divergence + y_divergence;

	divergence
}

/** Returns the cell solid modifiers (0 for solid, 1 otherwise) for cells in the order of: center,
	left, right, up, down. **/
fn calculate_cell_solids(grid: &SimGrid, cell_row: usize, cell_col: usize) -> [u8; 5] {

	/* Calculate collision modifiers for each cell face.  Note that we must perform a wrapping
		subtraction to prevent an underflow for our usize types. */
	let collision_center: u8	= grid.get_cell_type_value(cell_row, cell_col);
	let collision_left: u8		= grid.get_cell_type_value(usize::wrapping_sub(cell_col, 1), cell_col);
	let collision_right: u8		= grid.get_cell_type_value(cell_row, cell_col + 1);
	let collision_up: u8		= grid.get_cell_type_value(cell_row, usize::wrapping_sub(cell_row, 1));
	let collision_down: u8		= grid.get_cell_type_value(cell_row + 1, cell_col);

	[collision_center, collision_left, collision_right, collision_up, collision_down]
}
