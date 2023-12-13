use bevy::prelude::*;
use bevy::math::Vec2;
use crate::error::Error;
use crate::juice_renderer;

pub type Result<T> = core::result::Result<T, Error>;

pub struct SimStateManager;
impl Plugin for SimStateManager {
	
	fn build(&self, app: &mut App) {
		app.insert_resource(SimConstraints::default());
		app.insert_resource(SimGrid::default());

		app.add_systems(Startup, setup);
		app.add_systems(Update, update);
	}
}

/// Simulation state manager initialization.
fn setup(
	mut commands:		Commands,
	mut _constraints:	ResMut<SimConstraints>,
	mut _grid:			ResMut<SimGrid>) {
	
	let _test_particle = add_particle(&mut commands, Vec2::ZERO, Vec2::ONE);
	// let _test_particle = add_particle(commands, Vec2 { x: 10.0, y: 94.0 }, Vec2::ONE);
	
	// TODO: Get saved simulation data from most recently open file OR default file.
	// TODO: Population constraints, grid, and particles with loaded data.
}

/// Simulation state manager update; handles user interactions with the simulation.
fn update(
	mut _commands:		Commands,
	mut _constraints:	ResMut<SimConstraints>,
	mut _grid:			ResMut<SimGrid>) {

	// TODO: Check for and handle simulation saving/loading.
	// TODO: Check for and handle simulation pause/timestep change.
	// TODO: Check for and handle changes to simulation grid.
	// TODO: Check for and handle changes to gravity.
	// TODO: Check for and handle tool usage.
}

#[derive(Resource)]
struct SimConstraints {
	grid_particle_ratio:	f32, 	// PIC/FLIP simulation ratio.
	iterations_per_frame:	u8, 	// Simulation iterations per frame.
	gravity:				Vec2,	// Cartesian gravity vector.
}

impl Default for SimConstraints {
	
	fn default() -> SimConstraints {
		SimConstraints {
			grid_particle_ratio:	0.1,
			iterations_per_frame:	5,
			gravity:				Vec2 { x: 0.0, y: -9.81 },
		}
	}
}

impl SimConstraints {
	/// Change the gravity direction and strength constraints within the simulation.
	fn change_gravity(sim: &mut SimConstraints, gravity: Vec2) {
		sim.gravity = gravity;
	}

	// Toggle Timestep from defualt and zero value
	fn toggle_simulation_pause(sim: &mut SimConstraints) {
		if sim.iterations_per_frame != 0 {
			sim.iterations_per_frame = 0;
		}
		else{
			sim.iterations_per_frame = 5;// TODO: Create a variable to represent last speed set by user
		}
	}

	// Changes timestep of simulation
	fn change_timestep(sim: &mut SimConstraints, new_timstep: u8) {
		sim.iterations_per_frame = new_timstep;
	}
}

#[derive(Clone)]
enum SimGridCellType	{ Air, Fluid, Solid, }

#[derive(Resource)]
struct SimGrid {
	dimensions:	    (u16, u16),
	cell_size:		u16,
	cell_type:		Vec<SimGridCellType>,
	velocity:		Vec<[Vec2; 4]>,
}

impl Default for SimGrid {
	
	fn default() -> SimGrid {
		SimGrid {
			dimensions:	    (250, 250),
			cell_size:		10,
			cell_type:		vec![SimGridCellType::Air; 625],
			velocity:		vec![[Vec2::new(0.0, 0.0); 4]; 625],
		}
	}
}

impl SimGrid {
	/// Set simulation grid cell type.
    pub fn set_grid_cell_type(
        &mut self,
        cell_index: usize,
        cell_type: SimGridCellType) -> Result<()> {

        self.cell_type[cell_index] = cell_type;
        Ok(())
    }
	
	/// Set simulation grid dimensions.
    pub fn set_grid_dimensions(
        &mut self,
        width: u16,
        height: u16) -> Result<()> {

        if width % self.cell_size != 0 {
            return Err(Error::GridSizeError("Width not evenly divisible by cell size."));
        }

        if height % self.cell_size != 0 {
            return Err(Error::GridSizeError("Height not evenly divisible by cell size."));
        }

        self.dimensions = (width, height);

        Ok(())
    }
	
	// Set simulation grid cell size.
    pub fn set_grid_cell_size(
        &mut self,
        cell_size: u16) -> Result<()> {

        if self.dimensions.0 % cell_size != 0 {
            return Err(Error::GridSizeError("Grid cell size doesn't fit dimensions."))
        }

        if self.dimensions.1 % cell_size != 0 {
            return Err(Error::GridSizeError("Grid cell size doesn't fit dimensions."))
        }

        self.cell_size = cell_size;

        Ok(())
    }
}

#[derive(Component)]
pub struct SimParticle {
	pub position:	Vec2, 	// This particle's [x, y] position.
	pub velocity:	Vec2, 	// This particle's [x, y] velocity.
}

/// Add particles into the simulation.
fn add_particle(
	mut commands:	&mut Commands,
	position:		Vec2,
	velocity:		Vec2) -> Result<()> {
	
	let particle: Entity = commands.spawn(
		SimParticle {
			position:	position,
			velocity:	velocity,
		}
	).id();
	
	// IMPORTANT: Links a sprite to each particle for rendering.
	juice_renderer::link_particle_sprite(commands, particle);

	Ok(())
}

/// Remove a particle with ID particle_id from the simulation.
fn delete_particles(
	mut commands:	&mut Commands,
	mut particles:	Query<(Entity, &mut SimParticle)>,
	particle_id:	Entity) -> Result<()> {
	
	commands.entity(particle_id).despawn();
	Ok(())
}

/** Returns a vector of ID's of the particles within a circle centered at "position" with radius
	"radius." */
fn select_particles(
	particles:	Query<(Entity, &mut SimParticle)>,
	position:	Vec2,
	radius:		u32) -> Result<Vec<Entity>> {
	
	let mut selected_particles: Vec<Entity> = Vec::new();

	for (entity_id, particle) in particles.iter() {
		let distance: f32 = position.distance(particle.position);
		if distance <= (radius as f32) {
			selected_particles.push(entity_id);
		}
	}
	
	if selected_particles.len() > 0 {
		Ok(selected_particles)
	} else {
		Err(Error::NoParticlesFound("cannot select any particles!"))
	}
}