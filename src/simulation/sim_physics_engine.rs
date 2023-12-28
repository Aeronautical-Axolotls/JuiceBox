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

fn grid_to_particles(grid: ResMut<SimGrid>, particles: Query<(Entity, &mut SimParticle)>, flip_pic_coef: f32) -> Result<()> {



    Ok(())
}
