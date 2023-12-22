use bevy::prelude::*;
use crate::error::Error;
use super::sim_state_manager::{SimGrid, SimParticle};
use super::util::*;

pub type Result<T> = core::result::Result<T, Error>;

fn particles_to_grid(mut grid: ResMut<SimGrid>, particles: Query<(Entity, &mut SimParticle)>) {

    // for velocity_u points and velocity_v points,
    // add up all particle velocities nearby scaled
    // by their distance / cell width (their influence)
    // then divide by the summation of all their
    // influences

    let mut velocity_u = grid.velocity_u.clone();
    let mut velocity_v = grid.velocity_v.clone();

    for (row_index, row) in grid.velocity_u.iter().enumerate() {
        for (col_index, column) in grid.velocity_u[row_index].iter().enumerate() {

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

    for (row_index, row) in grid.velocity_v.iter().enumerate() {
        for (col_index, column) in grid.velocity_v[row_index].iter().enumerate() {

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

            velocity_u[row_index][col_index] = scaled_velocity_sum / scaled_influence_sum;
        }
    }

    grid.velocity_u = velocity_u;
    grid.velocity_v = velocity_v;

}

/// Force velocity incompressibility for each grid cell within the simulation.
pub fn make_grid_velocities_incompressible(mut grid: ResMut<SimGrid>) {
	
	// For each grid cell, make the fluid incompressible.
	for x in 0..grid.dimensions.0 {
		for y in 0..grid.dimensions.1 {
			
			// Calculate divergence for this cell.
			let left_flow: f32	= grid.velocity_u[x as usize][y as usize];
			let right_flow: f32	= grid.velocity_u[(x + 1) as usize][y as usize];
			let up_flow: f32	= grid.velocity_v[x as usize][y as usize];
			let down_flow: f32	= grid.velocity_v[x as usize][(y + 1) as usize];
			let divergence: f32	= calculate_divergence(left_flow, right_flow, up_flow, down_flow);
			
			
			
			// =================================
			// TODO: Factor in solid collisions.
			// =================================
			
			
			
			// Factor in any collisions with surrounding solid cells.
			// let solid_factor: f32			= 
			let divergence_per_side: f32	= divergence / 4.0;
			
			// Force incompressibility on this cell.
			grid.velocity_u[x as usize][y as usize]			+= divergence_per_side;
			grid.velocity_u[(x + 1) as usize][y as usize]	-= divergence_per_side;
			grid.velocity_v[x as usize][y as usize]			-= divergence_per_side;
			grid.velocity_v[x as usize][(y + 1) as usize]	+= divergence_per_side;
		}
	}
}

/** Calculate the divergence (inflow/outflow) of a grid cell.  If this number is not zero, then 
	the fluid must be made incompressible.  **A negative divergence indicates there is too much 
	inflow, whereas a positive divergence indicates too much outflow.** */
fn calculate_divergence(left_flow: f32, right_flow: f32, up_flow: f32, down_flow: f32) -> f32 {
	
	// BUG: The up and down flows may need to be reversed.
	let divergence: f32 = (right_flow - left_flow) + (up_flow - down_flow);
	divergence
}