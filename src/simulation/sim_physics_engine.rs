use bevy::prelude::*;
use crate::error::Error;
use super::{SimGrid, SimParticle, SimConstraints};
use super::util::*;

pub type Result<T> = core::result::Result<T, Error>;

pub fn particles_to_grid(grid: &mut SimGrid, particles: Vec<(Entity, &mut SimParticle)>) -> SimGrid {

    // for velocity_u points and velocity_v points,
    // add up all particle velocities nearby scaled
    // by their distance / cell width (their influence)
    // then divide by the summation of all their
    // influences

    // This function, after applying particle velocities
    // to the grid, returns a copy of the grid, but the
    // values are all "change in" values

    let mut velocity_u = grid.velocity_u.clone();
    let mut velocity_v = grid.velocity_v.clone();

    for row_index in 0..grid.velocity_u.len() {
        for col_index in 0..grid.velocity_u[row_index].len() {

            let pos = grid.get_velocity_point_pos(
                row_index,
                col_index,
                true);

            let mut scaled_velocity_sum = 0.0;

            let mut scaled_influence_sum = 0.0;

            for (id, particle) in particles.iter() {

                let influence = find_influence(
                    particle.position[0],
                    pos[0],
                    grid.cell_size);

                scaled_influence_sum += influence;
                scaled_velocity_sum += particle.velocity[0] * influence;
            }

            velocity_u[row_index][col_index] = scaled_velocity_sum / scaled_influence_sum;
        }
    }

    for row_index in 0..grid.velocity_v.len() {
        for col_index in 0..grid.velocity_v[row_index].len() {

            let pos = grid.get_velocity_point_pos(
                row_index,
                col_index,
                false);

            let mut scaled_velocity_sum = 0.0;

            let mut scaled_influence_sum = 0.0;

            for (id, particle) in particles.iter() {

                let influence = find_influence(
                    particle.position[1],
                    pos[1],
                    grid.cell_size);

                scaled_influence_sum += influence;
                scaled_velocity_sum += particle.velocity[1] * influence;
            }

            velocity_v[row_index][col_index] = scaled_velocity_sum / scaled_influence_sum;
        }
    }

    let old_grid = grid.clone();

    grid.velocity_u = velocity_u;
    grid.velocity_v = velocity_v;

    create_change_grid(&old_grid, &grid)

}

fn create_change_grid(old_grid: &SimGrid, new_grid: &SimGrid) -> SimGrid {

    let mut change_grid = old_grid.clone();
    let mut change_u: Vec<Vec<f32>> = Vec::new();
    let mut change_v: Vec<Vec<f32>> = Vec::new();

    for row_index in 0..change_grid.velocity_u.len() {
        for col_index in 0..change_grid.velocity_u[row_index].len() {
            change_u[row_index][col_index] = new_grid.velocity_u[row_index][col_index] - old_grid.velocity_u[row_index][col_index];
        }
    }

    for row_index in 0..change_grid.velocity_v.len() {
        for col_index in 0..change_grid.velocity_v[row_index].len() {
            change_v[row_index][col_index] = new_grid.velocity_v[row_index][col_index] - old_grid.velocity_v[row_index][col_index];
        }
    }

    change_grid.velocity_u = change_u;
    change_grid.velocity_v = change_v;

    change_grid

}

fn collect_particles<'a>(
        grid: &SimGrid,
        center: Vec2,
        particles: &'a Vec<(Entity, &mut SimParticle)>
    ) -> Vec<&'a (Entity, &'a mut SimParticle)> {

    let cell_dim = grid.cell_size as f32;

    let collection_radius = cell_dim / 2.0;

    let mut particle_bag = Vec::new();

    for particle in particles {
        let pos = Vec2::from((particle.1.position[0], particle.1.position[1]));
        let distance = center.distance(pos);
        if distance < cell_dim / collection_radius {
            particle_bag.push(particle);
        }
    }

    particle_bag

}

fn apply_grid(
        cell_size: u16,
        pos: Vec2,
        particles: Vec<&(Entity, &mut SimParticle)>,
        velocities: Vec<f32>,
        changes: Vec<f32>,
        pic_coef: f32,
    ) {

    let half_cell = cell_size as f32 / 2.0;
    let left_u = Vec2::new(pos[0] - half_cell, pos[1]);
    let right_u = Vec2::new(pos[0] + half_cell, pos[1]);
    let top_v = Vec2::new(pos[0], pos[1] + half_cell);
    let bottom_v = Vec2::new(pos[0], pos[1] - half_cell);

    for particle in particles {
        let new_velocity_u = (
                pic_coef * linear_interpolate(
                    left_u,
                    right_u,
                    velocities[0],
                    velocities[2],
                    particle.1.position,
                    true
                )
            ) + (
                (1.0 - pic_coef) * (
                    particle.1.velocity[0] + linear_interpolate(
                        left_u,
                        right_u,
                        changes[0],
                        changes[2],
                        particle.1.position,
                        true
                    )
                )
            );

        let new_velocity_v =
            (pic_coef * linear_interpolate(
                    top_v,
                    bottom_v,
                    velocities[1],
                    velocities[3],
                    particle.1.position,
                    false
                )
            ) + (
                (1.0 - pic_coef) * (
                    particle.1.velocity[1] + linear_interpolate(
                        top_v,
                        bottom_v,
                        changes[1],
                        changes[3],
                        particle.1.position,
                        false
                    )
                )
            );
    }

}

fn grid_to_particles(
        grid: &SimGrid,
        change_grid: &SimGrid,
        particles: Vec<(Entity, &mut SimParticle)>,
        flip_pic_coef: f32
    ) -> Result<()> {

    // Basic idea right now is to go through each cell,
    // figure out which particles are 'within' that cell,
    // then apply the grid transformation

    let half_cell = grid.cell_size as f32 / 2.0;
    let height = (grid.dimensions.1 * grid.cell_size) as f32;

    for row_index in 0..grid.dimensions.1 as usize {
        for col_index in 0..grid.dimensions.0 as usize {

            let pos = Vec2::new((col_index as f32 * grid.cell_size as f32) + half_cell, height - (row_index as f32 * grid.cell_size as f32 + half_cell));

            let particles_in_cell = collect_particles(grid, pos, &particles);

            let velocities = vec![
                grid.velocity_u[row_index][col_index],
                grid.velocity_v[row_index][col_index],
                grid.velocity_u[row_index][col_index + 1],
                grid.velocity_v[row_index+1][col_index]
            ];

            let changes = vec![
                change_grid.velocity_u[row_index][col_index],
                change_grid.velocity_v[row_index][col_index],
                change_grid.velocity_u[row_index][col_index + 1],
                change_grid.velocity_v[row_index+1][col_index]
            ];

            apply_grid(grid.cell_size, pos, particles_in_cell, velocities, changes, flip_pic_coef);
        }
    }

    Ok(())
}

/** Force velocity incompressibility for each grid cell within the simulation.  Uses the
	Gauss-Seidel method. */
pub fn make_grid_velocities_incompressible(
	grid: &mut SimGrid,
	constraints: &SimConstraints) {





	// ==============================================================
	// TODO: Adjust divergence based on particle density in the cell.
	// ==============================================================





	// Allows the user to make the simulation go BRRRRRRR or brrr.
	for _ in 0..constraints.iterations_per_frame {

		/* For each grid cell, calculate the inflow/outflow (divergence), find out how many surrounding
			cells are solid, then adjust grid velocities accordingly. */
		for row in 0..grid.dimensions.0 {
			for col in 0..grid.dimensions.1 {

				// Used to increase convergence time for our Gauss-Seidel implementation.
				let overrelaxation: f32	= 1.99;
				let divergence: f32		= calculate_cell_divergence(
					&grid,
					row as usize,
					col as usize,
					overrelaxation
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
				grid.velocity_v[row as usize][col as usize]			+= divergence * up_solid;
				grid.velocity_v[(row + 1) as usize][col as usize]	-= divergence * down_solid;
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
	grid: &SimGrid,
	cell_row: usize,
	cell_col: usize,
	overrelaxation: f32
) -> f32 {

	/* Retrieve velocities for each face of the current cell.  Note: this will not go out of
		bounds of the velocity arrays; each array is guaranteed to have sufficient space allocated
		to index like this. */
	let left_velocity: f32	= grid.velocity_u[cell_row][cell_col];
	let right_velocity: f32	= grid.velocity_u[cell_row][cell_col + 1];
	let up_velocity: f32	= grid.velocity_v[cell_row + 1][cell_col];
	let down_velocity: f32	= grid.velocity_v[cell_row][cell_col];
	// BUG: The up and down flows may need to be reversed.
	let x_divergence: f32	= right_velocity - left_velocity;
	let y_divergence: f32	= up_velocity - down_velocity;
	let divergence: f32		= overrelaxation * (x_divergence + y_divergence);
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
