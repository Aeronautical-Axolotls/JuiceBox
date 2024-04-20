pub mod sim_physics_engine;
pub mod sim_state_manager;
pub mod util;

use std::f32::consts::PI;

use bevy::prelude::*;
//use bevy::prelude::init_state;
use bevy::math::Vec2;
use crate::error::Error;
use crate::simulation::sim_state_manager::delete_all_faucets;
use crate::ui::{SimTool, UIStateManager};
use crate::util::{degrees_to_radians, polar_to_cartesian, cartesian_to_polar};
use sim_physics_engine::*;
use crate::test::test_state_manager::{self, construct_test_simulation_layout};
use crate::events::{PlayPauseStepEvent, ResetEvent, UseToolEvent};
use self::sim_state_manager::{activate_components, add_drain, add_faucet, add_particles_in_radius, delete_all_particles, delete_faucet, delete_particle, select_particles};

pub type Result<T> = core::result::Result<T, Error>;

pub struct Simulation;
impl Plugin for Simulation {

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

	construct_test_simulation_layout(constraints.as_mut(), grid.as_mut(), &mut commands);

	// TODO: Get saved simulation data from most recently open file OR default file.
	// TODO: Population constraints, grid, and particles with loaded data.
}

/// Simulation state manager update; handles user interactions with the simulation.
fn update(
	mut constraints:	ResMut<SimConstraints>,
	mut grid:			ResMut<SimGrid>,
	mut particles:		Query<(Entity, &mut SimParticle)>,
    faucets:		    Query<(Entity, &mut SimFaucet)>,
    drains:		        Query<(Entity, &mut SimDrain)>,
	keys:				Res<Input<KeyCode>>,

	mut commands:	Commands,
	asset_server:	Res<AssetServer>,
    ui_state:       Res<UIStateManager>,
    ev_tool_use: 	EventReader<UseToolEvent>,
    ev_reset:   	EventReader<ResetEvent>,
	ev_paused:		EventReader<PlayPauseStepEvent>) {

	// TODO: Check for and handle simulation saving/loading.

	/* A fixed timestep is generally recommended for fluid simulations like ours.  Unfortunately,
		this does mean that a lower framerate slows everything down, but it does prevent the
		whole thing from blowing up spectacularly.  For a dynamic timestep using the same scale of
		milliseconds, you would use the following code:
		let dynamic_timestep: f32 = time.delta().as_millis() as f32 * 0.001; */
	let fixed_timestep: f32 = constraints.timestep;

	// If the simulation is not paused, run the simulation!
	if !constraints.is_paused {
		step_simulation_once(
            &mut commands,
			constraints.as_mut(),
			grid.as_mut(),
			&mut particles,
            &faucets,
            &drains,
			fixed_timestep
		);
	}

	/* Handle all simulation events received through our EventReader<> objects.  IMPORTANT: This
		*must* happen after we step through the simulation.  If we handle events first, then in the
		case of a reset event, Bevy will not go through its despawn() schedule in time.  The
		simulation will then incorrectly label cells as fluid BEFORE the command to despawn the
		particles has executed.  Because the particles will be despawned before the next update
		schedule runs, there will never be a change in lookup index for these "ghost" particles, so
		they will not be removed from the simulation until the next reset event. */
	handle_events(
		ev_reset,
		ev_tool_use,
		ev_paused,
		&mut commands,
		&asset_server,
		constraints.as_mut(),
		grid.as_mut(),
		&mut particles,
		&faucets,
		&drains,
		&ui_state,
		fixed_timestep
	);
}

/// Handles incoming events from the UI
fn handle_events(
    mut ev_reset:       EventReader<ResetEvent>,
    mut ev_tool_use:    EventReader<UseToolEvent>,
	mut ev_pause:		EventReader<PlayPauseStepEvent>,
	mut commands:	    &mut Commands,
	asset_server:		&AssetServer,
	constraints:	    &mut SimConstraints,
	grid:			    &mut SimGrid,
	particles:		    &mut Query<(Entity, &mut SimParticle)>,
	faucets:		    &Query<(Entity, &mut SimFaucet)>,
	drains:		        &Query<(Entity, &mut SimDrain)>,
    ui_state:			&UIStateManager,
	timestep:			f32) {

    // If there is a reset event sent, we reset the simulation.
    for _ in ev_reset.read() {
        reset_simulation_to_default(&mut commands, constraints, grid, particles, faucets, drains);
		construct_test_simulation_layout(constraints, grid, &mut commands);
		return;
    }

	// If we receive a play/pause/step event, process it!
	for ev in ev_pause.read() {

		// If the event is not a step event, simply pause or unpause the simulation.
		if !ev.is_step_event {
			constraints.is_paused = !constraints.is_paused;

			/* If the event IS a step event, we need to either step once, or we need to pause and
				step once (even though the user is unlikely to notice the difference between a
				pause vs. a step then a pause.  I would feel guilty not stepping after the pause.
				it's like someone ordering a bagel with whole milk cream cheese, but then all you
				have is low-fat cream cheese.  You could just sneak it past them, but would that be
				the right thing to do?  No!  It would not!  Instead, you go to the store and get the
				whole milk cream cheese for them because you value their business and you want to
				do the right thing.  Ladies and gentlemen, I present to you: the cream cheese litmus
				test in the form of a physics engine UX design decision). */
		} else {

			if !constraints.is_paused {
				constraints.is_paused = true;
			}
			step_simulation_once(commands, constraints, grid, particles, faucets, drains, timestep);
		}
	}

    // For every tool usage, we change the state
    for tool_use in ev_tool_use.read() {

		if !grid.is_position_within_grid(&tool_use.pos) { continue; }
		let cell_coordinates: Vec2 = grid.get_cell_coordinates_from_position(&tool_use.pos);

        match tool_use.tool {
            SimTool::Select => {
                // TODO: Handle Select usage
            }
            SimTool::Grab => {
                // TODO: Handle Grab usage.
            }
            SimTool::AddFluid => {
                // TODO: Handle Add Fluid usage
            }
            SimTool::RemoveFluid => {
                // TODO: Handle Remove Fluid usage
            }
            SimTool::AddWall => {
				let _ = grid.set_grid_cell_type(
					cell_coordinates.x as usize,
					cell_coordinates.y as usize,
					SimGridCellType::Solid
				);

				//Delete all particles in the cell we are turning into a solid.
				let lookup_index: usize = grid.get_lookup_index(cell_coordinates);
				grid.delete_all_particles_in_cell(
					&mut commands,
					constraints,
					&particles,
					lookup_index
				);
            }
            SimTool::RemoveWall => {
				let _ = grid.set_grid_cell_type(
					cell_coordinates.x as usize,
					cell_coordinates.y as usize,
					SimGridCellType::Air
				);
            }
            SimTool::AddDrain => {
                add_drain(&mut commands, &asset_server, grid, tool_use.pos, None, ui_state.drain_radius * grid.cell_size as f32, ui_state.drain_pressure).ok();
            }
            SimTool::RemoveDrain => {
                // TODO: Handle Remove Drain usage
            }
            SimTool::AddFaucet => {

                // convert the direction from degrees to radians
                let direction = degrees_to_radians(ui_state.faucet_direction);
                // convert the direction and pressure into cartesian vector, pressure is scaled
                let faucet_direciton = polar_to_cartesian(Vec2::new(ui_state.faucet_pressure * 10.0, direction));

                add_faucet(&mut commands, &asset_server, grid, tool_use.pos, None, ui_state.faucet_radius, faucet_direciton).ok();
            }
            SimTool::RemoveFaucet => {

                // Get closest faucet id
                for (faucet_id, faucet_props) in faucets.iter() {
                    if tool_use.pos.distance(faucet_props.position) <= (grid.cell_size as f32 * 3.0) {
                        // Delete the closest faucet
                        delete_faucet(&mut commands, faucets, faucet_id);
                        break;
                    }
                }

            }
			// We should not never ever wever get here:
			_ => {}
        }
    }
}

/// Change the direction and strength of gravity!
pub fn change_gravity(
	constraints:		&mut SimConstraints,
	magnitude_change:	f32,
	direction_change:	f32) {

	// Convert existing gravity to polar coordinates.
	let mut polar_gravity: Vec2	= cartesian_to_polar(constraints.gravity);
	polar_gravity.x				+= 200.0 * magnitude_change as f32 * constraints.timestep;
	polar_gravity.y				+= 4.0 * direction_change as f32 * constraints.timestep;

	/* Limit the magnitude of the vector to prevent ugly behavior near 0.0.  ADDITIONALLY: I (Kade)
		found a bug where if a polar vector has magnitude 0 the direction will automatically become
		0.  This is bad and wrong, so cap gravity super close to zero for this special case... */
	polar_gravity.x				= f32::max(0.00001, polar_gravity.x);
	constraints.gravity			= polar_to_cartesian(polar_gravity);
}

/// Step the fluid simulation one time!
pub fn step_simulation_once(
	commands:		&mut Commands,
	constraints:	&mut SimConstraints,
	grid:			&mut SimGrid,
	particles:		&mut Query<(Entity, &mut SimParticle)>,
	faucets:		&Query<(Entity, &mut SimFaucet)>,
	drains:		    &Query<(Entity, &mut SimDrain)>,
	timestep:		f32) {

    // Run drains and faucets, panics if something weird/bad happens
    activate_components(commands, constraints, particles, faucets, drains, grid).ok();

	/* Integrate particles, update their lookup indices, update grid density values, and process
		collisions. */
    update_particles(constraints, particles, grid, timestep);
    push_particles_apart(constraints, grid, particles);
    handle_particle_grid_collisions(constraints, grid, particles);

	/* Label grid cells, transfer particle velocities to the grid, project/diffuse/advect them,
		then transfer velocities back.  Finally, extrapolate velocities to smooth out the
		fluid-air boundary. */
	grid.label_cells();
	particles_to_grid(grid, particles);
    extrapolate_values(grid, 1);

    // Store a copy of the grid from the previous simulation step for "change grid" creation.
	let old_grid = grid.clone();

	/* Make fluid incompressible, find the difference in grid from before incompressibility,
		interpolate grid velocities back to each particle, and finally extrapolate velocity values
		one final time! */
    make_grid_velocities_incompressible(grid, constraints);
    let change_grid = create_change_grid(&old_grid, &grid);
    grid_to_particles(grid, &change_grid, particles, constraints);
    extrapolate_values(grid, 1);
}

/// Reset simulation components to their default state and delete all particles.
pub fn reset_simulation_to_default(
	commands:			&mut Commands,
	constraints:		&mut SimConstraints,
	grid:				&mut SimGrid,
	particles:			&Query<(Entity, &mut SimParticle)>,
	faucets:			&Query<(Entity, &mut SimFaucet)>,
	drains:				&Query<(Entity, &mut SimDrain)>) {

	println!("Resetting simulation to default...");

	// Reset all particles, faucets, and drains!
	delete_all_particles(commands, constraints, grid, particles);
	delete_all_faucets(commands, faucets);
	// delete_all_drains(commands, drains);

	// Reset the grid by creating a new default grid and copying its values.
	let reset_grid: SimGrid	= SimGrid::default();
	let row_count: usize	= reset_grid.dimensions.0 as usize;
	let col_count: usize	= reset_grid.dimensions.1 as usize;
	grid.dimensions			= reset_grid.dimensions;
	grid.cell_size			= reset_grid.cell_size;
	grid.cell_type			= vec![vec![SimGridCellType::Air; row_count]; col_count];
	grid.cell_center		= vec![vec![0.0; row_count]; col_count];
	grid.velocity_u			= vec![vec![0.0; row_count + 1]; col_count];
	grid.velocity_v			= vec![vec![0.0; row_count]; col_count + 1];
	grid.spatial_lookup		= vec![vec![Entity::PLACEHOLDER; 0]; row_count * col_count];
	grid.density			= vec![0.0; row_count * col_count];
	println!("{:?}", grid.spatial_lookup);

	// Reset constraints by creating a default constraints and copying its values.
	let reset_constraints: SimConstraints	= SimConstraints::default();
	constraints.grid_particle_ratio			= reset_constraints.grid_particle_ratio;
	constraints.timestep					= reset_constraints.timestep;
	constraints.incomp_iters_per_frame		= reset_constraints.incomp_iters_per_frame;
	constraints.collision_iters_per_frame	= reset_constraints.collision_iters_per_frame;
	constraints.gravity						= reset_constraints.gravity;
	constraints.particle_radius				= reset_constraints.particle_radius;
	constraints.particle_count				= reset_constraints.particle_count;
	constraints.particle_rest_density		= reset_constraints.particle_rest_density;
}

#[derive(Resource, Reflect, Clone)]
#[reflect(Resource)]
pub struct SimConstraints {
	pub is_paused:					bool,	// Is the simulation currently paused?
	pub timestep:					f32,	// Timestep for simulation updates.
	pub gravity:					Vec2,	// Cartesian gravity vector.

	pub grid_particle_ratio:		f32, 	// PIC/FLIP simulation ratio (0.0 = FLIP, 1.0 = PIC).
	pub incomp_iters_per_frame:		u8, 	// Simulation incompressibility iterations per frame.
	pub collision_iters_per_frame:	u8,		// Collision iterations per frame.

	pub particle_radius:			f32,	// Particle collision radii.
	pub particle_count:				usize,	// Number of particles in the simulation.
	pub particle_rest_density:		f32,	// Rest density of particles in simulation.
}

impl Default for SimConstraints {

	fn default() -> SimConstraints {
		SimConstraints {
			is_paused:					false,
			timestep:					1.0 / 120.0,
			// (9.81 * 2) ^ 2 = ~385 (Bevy caps FPS at 60, we run sim at 120).
			gravity:					Vec2 { x: 0.0, y: -385.0 },

			grid_particle_ratio:		0.3,	// 0.0 = inviscid (FLIP), 1.0 = viscous (PIC).
			incomp_iters_per_frame:		100,
			collision_iters_per_frame:	2,

			particle_radius:			1.0,
			particle_count:				0,
			particle_rest_density:		0.0,
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
		if sim.incomp_iters_per_frame != 0 {
			sim.incomp_iters_per_frame = 0;
		}
		else{
			sim.incomp_iters_per_frame = 5;
            // TODO: Create a variable to represent last speed set by user
		}
	}

	// Changes number of iterations for incompressibility per frame.
	fn change_incompressibility_timestep(sim: &mut SimConstraints, new_timstep: u8) {
		sim.incomp_iters_per_frame = new_timstep;
	}

	// Changes number of iterations for particle collision per frame.
	fn change_collision_timestep(sim: &mut SimConstraints, new_timstep: u8) {
		sim.collision_iters_per_frame = new_timstep;
	}
}

#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub enum SimGridCellType {
	Solid,
    Fluid,
	Air,
}

#[derive(Clone, Debug, PartialEq, Eq, Reflect)]
pub enum SimSurfaceDirection {
    North,
    South,
    East,
    West,
}

#[derive(Resource, Clone, Reflect)]
#[reflect(Resource)]
pub struct SimGrid {
	pub	dimensions:	    (u16, u16),				// # of Hor. and Vert. cells in the simulation.
	pub	cell_size:		u16,
	pub	cell_type:		Vec<Vec<SimGridCellType>>,
	pub cell_center:    Vec<Vec<f32>>,			// Magnitude of pressure at center of cell.
	pub	velocity_u:		Vec<Vec<f32>>,			// Hor. magnitude as row<column<>>; left -> right.
	pub velocity_v:     Vec<Vec<f32>>,			// Vert. magnitude as row<column<>>; up -> down.
	pub spatial_lookup:	Vec<Vec<Entity>>,		// [cell_hash_value[list_of_entities_within_cell]].
	pub density:		Vec<f32>,				// Density for each grid cell.
}

impl Default for SimGrid {

	fn default() -> SimGrid {
		SimGrid {
			dimensions:	    (50, 50),
			cell_size:		5,
			cell_type:		vec![vec![SimGridCellType::Air; 50]; 50],
            cell_center:    vec![vec![0.0; 50]; 50],
			velocity_u:		vec![vec![0.0; 51]; 50],
            velocity_v:     vec![vec![0.0; 50]; 51],
			spatial_lookup:	vec![vec![Entity::PLACEHOLDER; 0]; 2500],
			density:		vec![0.0; 2500],
		}
	}
}

impl SimGrid {

	/// Create a new SimGrid!
	fn change_dimensions(&mut self, dimensions: (u16, u16), cell_size: u16) {

		let row_count: usize	= dimensions.0 as usize;
		let col_count: usize	= dimensions.1 as usize;

		self.dimensions			= dimensions;
		self.cell_size			= cell_size;
		self.cell_type			= vec![vec![SimGridCellType::Air; row_count]; col_count];
		self.cell_center		= vec![vec![0.0; row_count]; col_count];
		self.velocity_u			= vec![vec![0.0; row_count + 1]; col_count];
		self.velocity_v			= vec![vec![0.0; row_count]; col_count + 1];
		self.spatial_lookup		= vec![vec![Entity::PLACEHOLDER; 0]; row_count * col_count];
		self.density			= vec![0.0; row_count * col_count];
	}

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

        let grid_height = self.dimensions.0 * self.cell_size;
        let grid_length = self.dimensions.1 * self.cell_size;

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

	/** Convert the Vec2 position (x, y) to coordinates (row, column).  **will return the
		closest valid cell to any invalid position input.** */
	pub fn get_cell_coordinates_from_position(&self, position: &Vec2) -> Vec2 {
		let cell_size: f32			= self.cell_size as f32;
		let grid_upper_bound: f32	= self.dimensions.1 as f32 * cell_size;

		let mut coordinates: Vec2 = Vec2 {
			x: f32::floor((grid_upper_bound - position[1]) / cell_size),	// Row
			y: f32::floor(position[0] / cell_size),							// Column
		};

		// Clamp our coordinates to our grid's bounds.
		coordinates[0] = f32::max(0.0, coordinates[0]);
		coordinates[1] = f32::max(0.0, coordinates[1]);
		coordinates[0] = f32::min((self.dimensions.0 - 1) as f32, coordinates[0]);
		coordinates[1] = f32::min((self.dimensions.1 - 1) as f32, coordinates[1]);

		coordinates
	}



	/** Convert the Vec2 coordinates (row, column) to a position (x, y).  **will return the
		closest valid position to any invalid coordinate input.** */
	pub fn get_cell_position_from_coordinates(&self, coordinates: Vec2) -> Vec2 {
		let cell_size: f32			= self.cell_size as f32;
		let grid_max_x_bound: f32	= self.dimensions.1 as f32 * cell_size;
		let grid_max_y_bound: f32	= self.dimensions.0 as f32 * cell_size - cell_size;

		let mut position: Vec2 = Vec2 {
			x: f32::floor(coordinates.y * cell_size),
			y: f32::floor(grid_max_y_bound - coordinates.x * cell_size),
		};

		// Clamp our coordinates to our grid's bounds.
		position.x = f32::max(0.0, position.x);
		position.y = f32::max(0.0, position.y);
		position.x = f32::min(grid_max_x_bound, position.x);
		position.y = f32::min(grid_max_y_bound, position.y);

		position
	}

	/// Find the center position of a cell given its coordinates.
	pub fn get_cell_center_position_from_coordinates(&self, coordinates: &Vec2) -> Vec2 {
		let half_cell_size: f32	= (self.cell_size as f32) / 2.0;
		let cell_x: f32			= coordinates.y * self.cell_size as f32;
		let cell_y: f32			= coordinates.x * self.cell_size as f32;
		let grid_height: f32	= (self.dimensions.0 * self.cell_size) as f32;

		let cell_center_position: Vec2 = Vec2 {
			x: cell_x + half_cell_size,
			y: grid_height - cell_y - half_cell_size,
		};
		cell_center_position
	}

	/** Selects grid cells that entirely cover the a circle of radius `radius` centered at `position`;
		returns a Vector containing each cell's coordinates.  Note: the returned vector is of a
		static size.  If any cells in the selection are outside of the grid, then the closest valid
		cells will be added into the result.  **This can result in duplicated cell values, which is
		necessary to ensure accurate density calculations (corner cells would otherwise be
		considered much less dense than cells with selections entirely contained within the grid).**
		*/
	pub fn select_grid_cells(&self, position: Vec2, radius: f32) -> Vec<Vec2> {

		/* If we are less than a cell in radius, the function will only search 1 cell.  That is
			incorrect, as we could still need to search 4 cells if the selection is positioned
			properly.  Therefore, we cap the radius for selection-cell bound checking to 2.5, but
			leave the true radius untouched to retain proper particle selection behavior. */
		let min_selection_size: f32 = self.cell_size as f32 / 2.0;
		let adj_radius: f32			= f32::max(min_selection_size, radius);

		/* Find our min/max world coordinates for cells to search.  Add the cell size to account for
			the selection area potentially not being perfectly centered; this will ensure we always
			check the full possible number of cells our selection may be concerned with. We may check
			one or two extra cells, but I believe consistent behavior is worth 4 extra cell checks. */
		let selection_max_bound: Vec2 = Vec2 {
			x: position.x + adj_radius + self.cell_size as f32,
			y: position.y + adj_radius + self.cell_size as f32,
		};
		let selection_min_bound: Vec2 = Vec2 {
			x: position.x - adj_radius,
			y: position.y - adj_radius,
		};

		// Find the number of cells we need to check.
		let mut x_cell_count: usize			= (selection_max_bound.x - selection_min_bound.x) as usize;
		let mut y_cell_count: usize			= (selection_max_bound.y - selection_min_bound.y) as usize;
		x_cell_count						/= self.cell_size as usize;
		y_cell_count						/= self.cell_size as usize;
		let cells_in_selection_count: usize	= x_cell_count * y_cell_count;

		// Figure out which grid cells we are actually going to be checking.
		let mut cells_in_selection: Vec<Vec2>	= vec![Vec2::ZERO; cells_in_selection_count];
		for cell_index in 0..cells_in_selection_count {

			/* BUG: Sometimes the top two corner cells of the selection "flicker", and the sides have
				an extra cell jutting out.  Not sure why, but my guess is it's a type casting or
				rounding issue; not important (for now).  The corner flickering does affect the number
				of cells checked, however the extra cell jutting out does not (making me think the
				latter is a rendering issue).  Finally, the algorithm breaks down a little bit extra
				if the radius is not a multiple of the grid cell size. */

			// Per cell, get the x and y indices through our cell selection array.
			let cell_y_index: usize	= (cell_index / y_cell_count) % y_cell_count;
			let cell_x_index: usize	= cell_index % x_cell_count;

			// Convert the cell's x and y indices into a position, and then into a grid coordinate.
			let cell_position: Vec2 = Vec2 {
				x: selection_min_bound.x + cell_x_index as f32 * self.cell_size as f32,
				y: selection_min_bound.y + cell_y_index as f32 * self.cell_size as f32
			};

			if self.is_position_within_grid(&cell_position) {

				// Add our selected cell's coordinates to our list of selected cell coordinates!
				let cell_coordinates = self.get_cell_coordinates_from_position(&cell_position);
				cells_in_selection[cell_index] = cell_coordinates;
			}
		}

		cells_in_selection
	}

	/// Check if a position Vector is within the grid.
	pub fn is_position_within_grid(&self, position: &Vec2) -> bool {

		let max_x: f32 = (self.cell_size * self.dimensions.1) as f32;
		let max_y: f32 = (self.cell_size * self.dimensions.0) as f32;

		// Check position with grid bounds.
		if position.x < 0.0 || position.x > max_x { return false; }
		if position.y < 0.0 || position.y > max_y { return false; }

		true
	}

	/// Check if a coordinate Vector is within the grid.
	pub fn are_coordinates_within_grid(&self, coordinates: &Vec2) -> bool {

		if coordinates.x < 0.0 || coordinates.y >= self.dimensions.1 as f32 { return false; }
		if coordinates.y < 0.0 || coordinates.y >= self.dimensions.0 as f32 { return false; }

		true
	}

	/// Set all density values within the grid to 0.0.
	pub fn clear_density_values(&mut self) {
		for density in self.density.iter_mut() {
			*density = 0.0;
		}
	}

	/// Update each grid cell's density based on weighted particle influences.
	pub fn update_grid_density(&mut self, particle_position: Vec2) {

		/* Select all 9 nearby cells so we can weight their densities; a radius of 0.0
			automatically clamps to a 3x3 grid of cells surrounding the position vector. */
		let nearby_cells	= self.select_grid_cells(particle_position, 0.0);
		let center_cell		= self.get_cell_coordinates_from_position(&particle_position);

		// For each nearby cell, add weighted density value based on distance to particle_position.
		for cell in nearby_cells.iter() {
			let cell_lookup_index = self.get_lookup_index(*cell);

			// Get the center of the cell so we can weight density properly.
			let cell_position: Vec2		= self.get_cell_position_from_coordinates(*cell);
			let cell_center: Vec2		= Vec2 {
				x: cell_position.x + (0.5 * self.cell_size as f32),
				y: cell_position.y - (0.5 * self.cell_size as f32)
			};

			/* Weight density based on the center cell's distance to neighbors.  Distance squared
				to save ourselves the sqrt(); density is arbitrary here anyways. */
			let density_weight: f32 = f32::max(1.0, center_cell.distance_squared(*cell));
			self.density[cell_lookup_index]	+= 1.0 / density_weight;
		}
	}

	/// Gets an interpolated density value for a lookup index within the grid's bounds.
	pub fn get_density_at_position(&self, position: Vec2) -> f32 {

		let mut density: f32 = 0.0;

		// Select all 9 nearby cells so we can query their densities.
		let nearby_cells	= self.select_grid_cells(position, 0.0);
		let center_cell		= self.get_cell_coordinates_from_position(&position);

		// For each nearby cell, add its density weighted based on position to final density value.
		for mut cell in nearby_cells.iter() {

			// If one of our cell is solid, use the center cell's density instead.
			// if self.cell_type[cell.x as usize][cell.y as usize] == SimGridCellType::Solid {
			// 	cell = &center_cell;
			// }

			/* Weight density based on the center cell's distance to neighbors.  Distance squared
				to save ourselves the sqrt(); density is arbitrary here anyways. */
			let cell_lookup_index = self.get_lookup_index(*cell);
			let density_weight: f32 = f32::max(1.0, center_cell.distance_squared(*cell));
			density += self.density[cell_lookup_index] / density_weight;
		}

		density
	}

	// Get a cell lookup index into our spatial lookup table.
	pub fn get_lookup_index(&self, cell_coordinates: Vec2) -> usize {
		(cell_coordinates[1] as u16 + (cell_coordinates[0] as u16 * self.dimensions.0)) as usize
	}


	/// Add a new particle into our spatial lookup table.
	pub fn add_particle_to_lookup(&mut self, particle_id: Entity, lookup_index: usize) {

		if lookup_index > self.spatial_lookup.len() {
			eprintln!("Particle lookup index is out-of-bounds; cannot add particle to table!");
			return;
		}
		self.spatial_lookup[lookup_index].push(particle_id);
	}

	/// Remove a particle from our spatial lookup table; does nothing if the particle isn't found.
	pub fn remove_particle_from_lookup(&mut self, particle_id: Entity, lookup_index: usize) {

		if lookup_index > self.spatial_lookup.len() {
			eprintln!("Particle lookup index is out-of-bounds; cannot remove particle from table!");
			return;
		}

		// Search through our spatial lookup at the specified location.
		for particle_index in 0..self.spatial_lookup[lookup_index].len() {

			// If we found it, remove it.
			if self.spatial_lookup[lookup_index][particle_index] == particle_id {
				self.spatial_lookup[lookup_index].swap_remove(particle_index);
				break;
			}
		}
	}

	/// Get a Vec<Entity> of the particles currently inside of the cell at lookup_index.
	pub fn get_particles_in_lookup(&self, lookup_index: usize) -> Vec<Entity> {

		// Return an empty vector if we are out of bounds.
		if lookup_index >= (self.dimensions.0 * self.dimensions.1) as usize {
			return Vec::new();
		}

		let mut lookup_vector: Vec<Entity> = Vec::new();

		for particle_id in self.spatial_lookup[lookup_index].iter() {

			// TODO: Don't use placeholder!  Bad kitty!!!
			if *particle_id == Entity::PLACEHOLDER {
				continue;
			}

			lookup_vector.push(*particle_id);
		}

		lookup_vector
	}

	/// Delete all particles within a cell, given that cell's lookup index.
	pub fn delete_all_particles_in_cell(&mut self, commands: &mut Commands, constraints: &mut SimConstraints, particles: &Query<(Entity, &mut SimParticle)>, lookup_index: usize) {

		for particle_id in self.spatial_lookup[lookup_index].iter_mut() {
			// Look for the particle in our particles query.
			if let Ok(particle) = particles.get(*particle_id) {

				/* Despawn particle; since we are already mutably borrowing the lookup table, we
					can't remove any particles from the lookup table until we are done iterating
					through the table. */
				commands.entity(*particle_id).despawn();

				/* BUG: This overflowed once while testing, and I'm betting it's because I misuse
					Entity::PLACEHOLDER.  Here is my silly little fix: */
				if constraints.particle_count > 0 {
					constraints.particle_count -= 1;
				}
			}
		}

		// Clear the spatial lookup table at the current index.
		self.spatial_lookup[lookup_index].clear();
	}

    /// Get velocity of the cell
    pub fn get_cell_velocity(&self, row: usize, column: usize) -> Vec2 {

        if row as u16 >= self.dimensions.0 || column as u16 >= self.dimensions.1 || row == 0 || column == 0 {
            return Vec2::ZERO;
        }

        let left_u = self.velocity_u[row][column];
        let right_u = self.velocity_u[row][column + 1];
        let top_v = self.velocity_v[row][column];
        let down_v = self.velocity_v[row + 1][column];

        let u_avg = (left_u + right_u) / 2.0;
        let v_avg = (top_v + down_v) / 2.0;

        let velocity = Vec2::new(u_avg, v_avg);

        velocity

    }

	/// Get the particles in all 9 cells surrounding a point.
	fn get_nearby_particles(&self, lookup_index: usize) -> Vec<Entity> {

		let mut nearby_particles: Vec<Entity>	= Vec::new();
		let mut cells_to_check: Vec<usize>		= Vec::new();
		let col_count: usize					= self.dimensions.1 as usize;

		let is_cell_on_right_border: bool		= lookup_index % (col_count - 1) == 0;
		let is_cell_on_left_border: bool		= lookup_index % col_count == 0;

		/* Make sure the current row's cells-to-check are valid.  If they are, search for particles
			within them. */
		cells_to_check.push(lookup_index);
		if lookup_index > 0 && !is_cell_on_left_border {
			cells_to_check.push(lookup_index - 1);
		}
		if lookup_index < self.spatial_lookup.len() && !is_cell_on_right_border {
			cells_to_check.push(lookup_index + 1);
		}

		// Previous row's cell check:
		if lookup_index >= col_count {
			cells_to_check.push(lookup_index - col_count);
			if !is_cell_on_left_border {
				cells_to_check.push(lookup_index - col_count - 1);
			}
			if !is_cell_on_right_border {
				cells_to_check.push(lookup_index - col_count + 1);
			}
		}

		// Next row's cell check:
		if lookup_index <= self.spatial_lookup.len() - col_count {
			cells_to_check.push(lookup_index + col_count);
			if !is_cell_on_left_border {
				cells_to_check.push(lookup_index + col_count - 1);
			}
			if lookup_index < self.spatial_lookup.len() - col_count && !is_cell_on_right_border {
				cells_to_check.push(lookup_index + col_count + 1);
			}
		}


		for i in 0..cells_to_check.len() {
			nearby_particles.append(&mut self.get_particles_in_lookup(cells_to_check[i]));
		}

		nearby_particles
	}

    /**
        Goes through the entire grid and labels the cells with their respective type
    **/
    pub fn label_cells(&mut self) {

        let (rows, cols) = self.dimensions;

        // Create a new label array
        let mut cell_types = vec![vec![SimGridCellType::Air; cols as usize]; rows as usize];

        for row in 0..rows as usize {
            for col in 0..cols as usize {

                // Check if cell is solid
                if self.cell_type[row][col] == SimGridCellType::Solid {
                    cell_types[row][col] = SimGridCellType::Solid;
                    continue;
                }

                let lookup_index = self.get_lookup_index(Vec2::new(row as f32, col as f32));

                // Get the particles within the current cell
                let particles = self.get_particles_in_lookup(lookup_index);

                // Determine if non-solid cell is Air or fluid.
                if particles.len() == 0 {
                    cell_types[row][col] = SimGridCellType::Air;
                }
                else {
                    cell_types[row][col] = SimGridCellType::Fluid;
                }

            }
        }

        // Set the label array to new label area
        self.cell_type = cell_types;

    }

}

#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct SimParticle {
	pub position:		Vec2, 	// This particle's [x, y] position.
	pub velocity:		Vec2, 	// This particle's [x, y] velocity.
	pub lookup_index:	usize,	// Bucket index into spatial lookup for efficient neighbor search.
}

/// Faucet Object for simulation
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct SimFaucet {
    pub position:       Vec2,                           // Faucet Postion in the simulation
    pub direction:      Option<SimSurfaceDirection>,    // Direction to which the faucet is connected with the wall
    pub diameter:       f32,
    pub velocity:       Vec2,
}

impl SimFaucet {

    pub fn new(
        position: Vec2,
        direction: Option<SimSurfaceDirection>,
        diameter: f32,
        velocity: Vec2,
        ) -> Self {

        Self {
            position,
            direction,
            diameter,
            velocity,
        }
    }

    /// Runs the faucet, adds fluid particles, enforces solids
    pub fn run(
        &self,
        commands: &mut Commands,
        constraints: &mut SimConstraints,
        grid: &mut SimGrid
        ) -> Result<()> {

        let cell_coords = grid.get_cell_coordinates_from_position(&self.position);

        // Run fluid
        let position = self.position + Vec2::new(0.0, -(grid.cell_size as f32));
        add_particles_in_radius(commands, constraints, grid, self.diameter, self.diameter, position, self.velocity);

        Ok(())
    }
}

/// Drain Object for simulation
#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct SimDrain {
    pub position:       Vec2,                           // Drain Postion in the simulation
    pub direction:      Option<SimSurfaceDirection>,    // Direction to which the drain is connected with the wall
    pub radius:         f32,                            // Radius of the darin's pull
    pub pressure:       f32,                            // Magnitude of the drain's pull
}

impl SimDrain {

    /// New Drain
    pub fn new(
        position: Vec2,
        direction: Option<SimSurfaceDirection>,
        radius: f32,
        pressure: f32,
        ) -> Self {

        Self {
            position,
            direction,
            radius,
            pressure,
        }
    }

    /// Removes nearby particles
    pub fn drain(
        &self,
        commands: &mut Commands,
        constraints: &mut SimConstraints,
        grid: &mut SimGrid,
        particles: &mut Query<(Entity, &mut SimParticle)>,
        ) -> Result<()> {

        let nearby_particle_ids = select_particles(particles, grid, self.position, self.radius);

        for particle_id in nearby_particle_ids.iter() {

            let Ok((_, mut particle)) = particles.get_mut(*particle_id) else {
                continue;
            };

            let distance = self.position.distance(particle.position);
            let distance_vector = particle.position - self.position;
            let polar_vector = cartesian_to_polar(distance_vector); // (magnitude, direction)
            let pull_strength = self.pressure * (1.0 / polar_vector.x);

            // prevent particle from being pulled past the drain
            if distance < pull_strength {
                let _ = delete_particle(commands, constraints, particles, grid, *particle_id);
                continue;
            }

            let pull_direction = polar_vector.y + degrees_to_radians(180.0);
            let pull_velocity = polar_to_cartesian(Vec2::new(pull_strength, pull_direction));

            particle.position += pull_velocity;

            if distance < grid.cell_size as f32 * 1.5 {
                if let Err(_) = delete_particle(commands, constraints, particles, grid, *particle_id) {
                    continue;
                };
            }

        }

        Ok(())
    }
}
