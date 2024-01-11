use std::f32::consts::{PI, FRAC_2_PI, FRAC_PI_2, E, LOG2_E};

use bevy::prelude::*;
use bevy::math::Vec2;
use crate::error::Error;
use crate::juice_renderer;
use crate::test;

use super::sim_physics_engine::{
	particles_to_grid,
	make_grid_velocities_incompressible,
};

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
	mut constraints:	ResMut<SimConstraints>,
	mut grid:			ResMut<SimGrid>) {
	
	test::construct_test_simulation_layout(grid.as_mut(), commands);
	// TODO: Get saved simulation data from most recently open file OR default file.
	// TODO: Population constraints, grid, and particles with loaded data.
}

/// Simulation state manager update; handles user interactions with the simulation.
fn update(
	mut constraints:	ResMut<SimConstraints>,
	mut grid:			ResMut<SimGrid>) {

	// TODO: Check for and handle simulation saving/loading.
	// TODO: Check for and handle simulation pause/timestep change.
	// TODO: Check for and handle changes to simulation grid.
	make_grid_velocities_incompressible(grid.as_mut(), constraints.as_ref());
	// TODO: Check for and handle changes to gravity.
	// TODO: Check for and handle tool usage.
}

#[derive(Resource)]
pub struct SimConstraints {
	pub grid_particle_ratio:	f32, 	// PIC/FLIP simulation ratio.
	pub iterations_per_frame:	u8, 	// Simulation iterations per frame.
	pub gravity:				Vec2,	// Cartesian gravity vector.
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
			sim.iterations_per_frame = 5;
            // TODO: Create a variable to represent last speed set by user
		}
	}

	// Changes timestep of simulation
	fn change_timestep(sim: &mut SimConstraints, new_timstep: u8) {
		sim.iterations_per_frame = new_timstep;
	}
}

#[derive(Clone, Debug)]
pub enum SimGridCellType {
	Solid,
    Fluid,
	Air,
}

#[derive(Resource)]
pub struct SimGrid {
	pub	dimensions:	    (u16, u16),				// # of Hor. and Vert. cells in the simulation.
	pub	cell_size:		u16,
	pub	cell_type:		Vec<Vec<SimGridCellType>>,
	pub cell_center:    Vec<Vec<f32>>,			// Magnitude of pressure at center of cell.
	pub	velocity_u:		Vec<Vec<f32>>,			// Hor. magnitude as row<column<>>; left -> right.
	pub velocity_v:     Vec<Vec<f32>>,			// Vert. magnitude as row<column<>>; up -> down.
}

impl Default for SimGrid {

	fn default() -> SimGrid {
		SimGrid {
			dimensions:	    (25, 25),
			cell_size:		10,
			cell_type:		vec![vec![SimGridCellType::Air; 25]; 25],
            cell_center:    vec![vec![0.0; 25]; 25],
			velocity_u:		vec![vec![0.0; 26]; 25],
            velocity_v:     vec![vec![0.0; 25]; 26],
		}
	}
}

impl SimGrid {
	/// Set simulation grid cell type.
    pub fn set_grid_cell_type(
        &mut self,
        cell_x: usize,
		cell_y: usize,
        cell_type: SimGridCellType) -> Result<()> {

		if cell_x >= self.dimensions.0 as usize {
			return Err(Error::OutOfGridBounds("X-coord. is out of bounds!"));
		}
		if cell_y >= self.dimensions.1 as usize {
			return Err(Error::OutOfGridBounds("Y-coord. is out of bounds!"));
		}

        self.cell_type[cell_x][cell_y] = cell_type;

        Ok(())
    }

	/// Set simulation grid dimensions.
    pub fn set_grid_dimensions(
        &mut self,
        width: u16,
        height: u16) -> Result<()> {

        self.dimensions = (width, height);

        Ok(())
    }

	// Set simulation grid cell size.
    pub fn set_grid_cell_size(
        &mut self,
        cell_size: u16) -> Result<()> {

        self.cell_size = cell_size;

        Ok(())
    }

    pub fn get_velocity_point_pos(&self, row_index: usize, col_index: usize, horizontal: bool) -> Vec2 {
        // This function receives a row and column to index the point in either
        // `self.velocity_u` or `self.velocity_v` and find where their (x, y)
        // coords are.

        // Since the horizontal velocity points (u) have one more horizontally
        // and the vertical velocity points (v) have one more vertically,
        // the `horizontal` parameter is needed to differentiate between
        // `self.velocity_u` and `self.velocity_v`.

        let grid_length = self.dimensions.0 * self.cell_size;
        let grid_height = self.dimensions.1 * self.cell_size;

        let offset = (self.cell_size / 2) as f32;

        if horizontal {
            let pos_x = col_index as f32 * self.cell_size as f32;
            let pos_y = grid_height as f32 - (row_index as f32 * self.cell_size as f32 + offset);

            return Vec2::new(pos_x, pos_y);

        } else {
            let pos_x = col_index as f32 * self.cell_size as f32 + offset;
            let pos_y = grid_height as f32 - (row_index as f32 * self.cell_size as f32);

            return Vec2::new(pos_x, pos_y);
        }

    }

	/** Get the collision value of a cell; returns 0 if SimGridCellType::Solid OR if cell_x or
		cell_y are out of bounds.  Returns 1 if SimGridCellType::Fluid or SimGridCellType::Air. */
	pub fn get_cell_type_value(&self, cell_row: usize, cell_col: usize) -> u8 {

		// Because cell_x and cell_y are unsigned, we do not need an underflow check.
		if cell_row >= self.dimensions.0 as usize ||
			cell_col >= self.dimensions.1 as usize {
			return 0;
		}

		/* When modifying flow out of a cell, we need to modify said flow by 0 if the
			cell the flow is going into is solid.  If the cell is not solid, we leave flow
			unmodified. */
		match self.cell_type[cell_row][cell_col] {
			SimGridCellType::Solid	=> 0,
			SimGridCellType::Fluid	=> 1,
			SimGridCellType::Air	=> 1,
		}
	}

	/** Convert the Vec2 coordinates (row, column) from a position (x, y).  **Does not guarantee
		that the requested position for the cell is valid; only that if a cell were to exist
		at the given position, it would have the returned Vec2 as its (row, column)
		coordinates.** */
	pub fn get_cell_coordinates_from_position(&self, position: &Vec2) -> Vec2 {
		let cell_size: f32			= self.cell_size as f32;
		let grid_upper_bound: f32	= self.dimensions.0 as f32 * cell_size;
		let coordinates: Vec2 = Vec2 {
			x: (grid_upper_bound - position[1]) / cell_size,	// Row
			y: position[0] / cell_size,							// Column
		};

		coordinates
	}
}

#[derive(Component)]
pub struct SimParticle {
	pub position:	Vec2, 	// This particle's [x, y] position.
	pub velocity:	Vec2, 	// This particle's [x, y] velocity.
}

/** Add many particles into the simulation within a radius.  Note that particle_density is 
	the number of particles per unit radius. */
pub fn add_particles_in_radius(
	commands:			&mut Commands,
	grid:				&SimGrid,
	particle_density:	f32,
	radius:				f32,
	center_position:	Vec2,
	velocity:			Vec2) {
	
	// Create center particle.
	let _center_particle = add_particle(commands, grid, center_position, velocity);
	
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
			let _particle = add_particle(commands, grid, particle_position, velocity);
		}
	}
}

/// Add particles into the simulation.
fn add_particle(
	commands:	&mut Commands,
	grid:			&SimGrid,
	position:		Vec2,
	velocity:		Vec2) -> Result<()> {
	
	// Don't allow the user to create particles out of the simulation grid's bounds!
	if position[0] < 0.0 || position[0] > (grid.dimensions.1 * grid.cell_size) as f32 {
		return Err(Error::OutOfGridBounds("X-coordinate for particle creation is out of grid bounds!"));
	}
	if position[1] < 0.0 || position[1] > (grid.dimensions.0 * grid.cell_size) as f32 {
		return Err(Error::OutOfGridBounds("Y-coordinate for particle creation is out of grid bounds!"));
	}
	// If the cell we are inside of is a solid, don't create the particle!
	let cell_coordinates: Vec2 = grid.get_cell_coordinates_from_position(&position);
	if matches!(
		grid.cell_type[cell_coordinates[0] as usize][cell_coordinates[1] as usize],
		SimGridCellType::Solid) {
		return Err(Error::InvalidCellParticleCreation("Chosen cell is solid!"));
	}
	
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
pub fn select_particles(
	particles:	Query<(Entity, &mut SimParticle)>,
	position:	Vec2,
	radius:		u32) -> Result<Vec<Entity>> {

	/* TODO: Rework this function to use a spatial lookup based on SimGrid.  If a particle is
		outside of the nearest grid cells, then skip checking it.  We can accomplish this in a
		parallel-friendly way by sorting a list of spatial lookups for particles based on the grid,
		then choosing the nearest 1/9/25/49 grid cells (based on radius). */

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
