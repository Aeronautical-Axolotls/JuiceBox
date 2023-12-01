use bevy::prelude::*;
use bevy::math::Vec2;
use anyhow::Result;

pub struct SimStateManager;
impl Plugin for SimStateManager {
	fn build(&self, app: &mut App) {
		app.insert_resource(SimConstraints::default());
		app.insert_resource(SimParticles::default());
		app.insert_resource(SimGrid::default());
		
		app.add_systems(Startup, setup);
		app.add_systems(Update, update);
	}
}

/// Simulation state manager initialization.
fn setup(
	mut _commands:		Commands, 
	mut _constraints:	ResMut<SimConstraints>, 
	mut _grid:			ResMut<SimGrid>, 
	mut _particles:		ResMut<SimParticles>) {
	
	println!("Initializing state manager...");
	
	// TODO: Get saved simulation data from most recently open file OR default file.
	// TODO: Population constraints, grid, and particles with loaded data.
	
	println!("State manager initialized!");
}

/// Simulation state manager update; handles user interactions with the simulation.
fn update(
	mut _commands:		Commands, 
	mut _constraints:	ResMut<SimConstraints>, 
	mut _grid:			ResMut<SimGrid>, 
	mut _particles:		ResMut<SimParticles>) {
	
	// TODO: Check for and handle simulation saving/loading.
	// TODO: Check for and handle simulation pause/timestep change.
	// TODO: Check for and handle changes to simulation grid.
	// TODO: Check for and handle changes to gravity.
	// TODO: Check for and handle tool usage.
}

/** Add particles into the simulation, each with a position of positions[i] and velocities[i].  If 
	the list lengths do not match, the function will not add the particles to avoid unwanted 
	behavior. */
fn _add_particles(sim: &mut SimParticles, positions: &mut Vec<Vec2>, velocities: &mut Vec<Vec2>) {
	if positions.len() != velocities.len() {
		println!("Mismatched vector lengths; could not add particles!");
		return;
	}
	
	sim._particle_count += positions.len();
	sim.particle_position.append(positions);
	sim.particle_velocity.append(velocities);
}

/** Remove particles from the simulation, each with a particle index of indices[i].  If a value 
	within indices is out of range of the number of particles, the function will skip that 
	particle and continue on. */
fn _delete_particles(sim: &mut SimParticles, indices: Vec<usize>) {
	for i in 0..indices.len() {
		let particle_index: usize = indices[i];
		
		if particle_index >= sim.particle_position.len() {
			println!("Index out of range; particle {} not deleted!", i);
			continue;
		}
		
		sim.particle_position.remove(particle_index);
	}
}

/** Returns a vector of indices of the particles within a circle centered at "position" with radius 
	"radius." */
fn _select_particles(sim: &mut SimParticles, position: Vec2, radius: u32) -> Result<Vec<usize>> {
	let mut selected_particles: Vec<usize> = Vec::new();
	
	for i in 0..sim.particle_position.len() {
		let distance: f32 = position.distance(sim.particle_position[i]);
		if distance <= (radius as f32) {
			selected_particles.push(i);
		}
	}
	
	Ok(selected_particles)
}

/// Change the gravity direction and strength constraints within the simulation.
fn _change_gravity(sim: &mut SimConstraints, direction: u16, strength: f32)
{
	sim.gravity_direction = direction;
	sim.gravity_strength = strength;
}

#[derive(Resource)]
struct SimConstraints {
	_grid_particle_ratio:	f32, 	// PIC/FLIP simulation ratio.
	_iterations_per_frame:	u8, 	// Simulation iterations per frame.
	gravity_direction:		u16, 	// Gravity direction in degrees.
	gravity_strength:		f32, 	// Gravity strength in m/s^2.
}
impl Default for SimConstraints {
	fn default() -> SimConstraints {
		SimConstraints {
			_grid_particle_ratio:	0.1, 
			_iterations_per_frame:	5, 
			gravity_direction:		270, 
			gravity_strength:		9.81, 
		}
	}
}

enum SimGridCellType	{ Air, _Fluid, _Solid, }
#[derive(Resource)]
struct SimGrid {
	_dimensions:	[u16; 2], 			// Number of grid cells as [x, y].
	_cell_size:		u8, 				// Grid cell size (lower size -> higher precision).
	_cell_type:		SimGridCellType, 
	_velocity:		Vec<[Vec2; 4]>, 	// Velocities for each grid cell at all 4 edges.
}
impl Default for SimGrid {
	fn default() -> SimGrid {
		SimGrid {
			_dimensions:	[250, 250], 
			_cell_size:		10, 
			_cell_type:		SimGridCellType::Air, 
			
			// BUG: Not sure if this is correct.
			_velocity:		Vec::new(), 
		}
	}
}

#[derive(Resource)]
struct SimParticles {
	_particle_count:	usize, 		// Current number of particles.
	particle_position:	Vec<Vec2>, 	// Each particle's [x, y] position.
	particle_velocity:	Vec<Vec2>, 	// Each particle's [x, y] velocity.
}
impl Default for SimParticles {
	fn default() -> SimParticles {
		SimParticles {
			_particle_count:	0, 
			particle_position:	Vec::new(), 
			particle_velocity:	Vec::new(), 
		}
	}
}