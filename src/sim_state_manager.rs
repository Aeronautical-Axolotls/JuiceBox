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

// Simulation state manager initialization code.
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

// Simulation state manager update; handles user interactions with the simulation.
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

fn _add_particles(_positions: Vec<Vec2>) {
	// TODO: Add each position to the state manager's list of particle positions.
}

fn _delete_particles(_indices: Vec<u64>) {
	// TODO: Delete each particle whose index corresponds with a value in indices.
}

fn _select_particles(_indices: Vec<u64>) {
	// TODO: Select each particle whose index corresponds with a value in indices.
}

#[derive(Resource)]
struct SimConstraints {
	_grid_particle_ratio:	f32, 	// PIC/FLIP simulation ratio.
	_iterations_per_frame:	u8, 	// Simulation iterations per frame.
	_gravity_direction:		u16, 	// Gravity direction in degrees.
	_gravity_strength:		f32, 	// Gravity strength in m/s^2.
}
impl Default for SimConstraints {
	fn default() -> SimConstraints {
		SimConstraints {
			_grid_particle_ratio:	0.1,
			_iterations_per_frame:	5,
			_gravity_direction:		270,
			_gravity_strength:		9.81,
		}
	}
}

#[derive(Clone)]
enum SimGridCellType	{ Air, Fluid, Solid, }

#[derive(Resource)]
struct SimGrid {
	dimensions:	    [u16; 2],
	cell_size:		u8,
	cell_type:		Vec<SimGridCellType>,
	velocity:		Vec<[Vec2; 4]>,
}

impl Default for SimGrid {
	fn default() -> SimGrid {
		SimGrid {
			dimensions:	[250, 250],
			cell_size:		10,
			cell_type:		vec![SimGridCellType::Air; 625],
			velocity:		vec![[Vec2::new(0.0, 0.0); 4]; 625],
		}
	}
}

impl SimGrid {
    pub fn _set_grid_cell_type(&mut self, cell_index: usize, cell_type: SimGridCellType) -> Result<()> {
        self.cell_type[cell_index] = cell_type;
        Ok(())
    }
}

#[derive(Resource)]
struct SimParticles {
	_particle_count:	u64, 		// Current number of particles.
	_position:			Vec<Vec2>, 	// Each particle's [x, y] position.
	_velocity:			Vec<Vec2>, 	// Each particle's [x, y] velocity.
}
impl Default for SimParticles {
	fn default() -> SimParticles {
		SimParticles {
			_particle_count:	0,
			_position:			Vec::new(),
			_velocity:			Vec::new(),
		}
	}
}
