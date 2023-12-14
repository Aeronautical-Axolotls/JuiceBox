use bevy::prelude::*;
use super::{SimParticle, SimGrid};
use super::util::*;

pub struct SimPhysicsEngine;
impl Plugin for SimPhysicsEngine {

	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup);
		app.add_systems(Update, update);
	}
}

fn setup(mut _commands: Commands) {

}

fn update(mut _commands: Commands) {

}

fn particles_to_grid(grid: SimGrid, query: Query<SimParticle>) -> Result<()> {

    // for velocity_u points and velocity_v points,
    // add up all particle velocities nearby scaled
    // by their distance / cell width (their influence)
    // then divide by the summation of all their
    // influences

    for horizontal in grid.velocity_u {
        let particles = todo!(); // Particle selecting function to be written by Kade

        for particle in particles {
            let influence = todo!(); // Influence determined by find_influence in util

        }
    }

    Ok(())
}
