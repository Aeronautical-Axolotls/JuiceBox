---
title: Juicebox Dev Docs
---
# SRC

## Simulation

### <SwmPath>[src/simulation/mod.rs](/src/simulation/mod.rs)</SwmPath>

<SwmSnippet path="/src/simulation/mod.rs" line="27">

---

This code snippet initializes the simulation state manager by setting up the `constraints` and `grid`. It calls the `construct_test_simulation_layout` function from the `test_state_manager` module to populate the `constraints`, `grid`, and `commands` with test simulation data. It also contains two TODOs to get saved simulation data and populate constraints, grid, and particles with the loaded data.

```renderscript
/// Simulation state manager initialization.
fn setup(
	commands:			Commands,
	mut constraints:	ResMut<SimConstraints>,
	mut grid:			ResMut<SimGrid>) {

	test_state_manager::construct_test_simulation_layout(
		constraints.as_mut(),
		grid.as_mut(),
		commands
	);


	// TODO: Get saved simulation data from most recently open file OR default file.
	// TODO: Population constraints, grid, and particles with loaded data.
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="44">

---

This code snippet is an update function that manages the state of a simulation. It handles user interactions, such as saving/loading the simulation, pausing/timestep changing, gravity changes, and tool usage. It also calls the `step_simulation_once` function to advance the simulation once when certain conditions are met.

```renderscript
/// Simulation state manager update; handles user interactions with the simulation.
fn update(
	mut constraints:	ResMut<SimConstraints>,
	mut grid:			ResMut<SimGrid>,
	mut particles:		Query<(Entity, &mut SimParticle)>,
	keys:				Res<Input<KeyCode>>,

	mut commands:	Commands,
	mut gizmos:		Gizmos,
	windows:		Query<&Window>,
	cameras:		Query<(&Camera, &GlobalTransform)>
	) {

	// TODO: Check for and handle simulation saving/loading.
	// TODO: Check for and handle simulation pause/timestep change.

	// let delta_time: f32 = time.delta().as_millis() as f32 * 0.001;
	let fixed_timestep: f32 = constraints.timestep;

	// If F is not being held, run the simulation.
	if !keys.pressed(KeyCode::F) {
		step_simulation_once(
			constraints.as_mut(),
			grid.as_mut(),
			&mut particles,
			fixed_timestep
		);

		// If F is being held and G is tapped, step the simulation once.
	} else if keys.just_pressed(KeyCode::G) {
		step_simulation_once(
			constraints.as_mut(),
			grid.as_mut(),
			&mut particles,
			fixed_timestep
		);
	}

	// TODO: Check for and handle changes to gravity.
	// TODO: Check for and handle tool usage.
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="86">

---

This code snippet steps the fluid simulation one time. It integrates particles, updates their lookup indices, updates grid density values, and processes collisions. It also labels grid cells, transfers particle velocities to the grid, projects/diffuses/advects them, then transfers velocities back. It extrapolates velocities to smooth out the fluid-air boundary. It makes the fluid incompressible, finds the difference in grid from before incompressibility, interpolates grid velocities back to each particle, and finally extrapolates velocity values one final time.

```renderscript
/// Step the fluid simulation one time!
fn step_simulation_once(
	constraints:	&mut SimConstraints,
	grid:			&mut SimGrid,
	particles:		&mut Query<(Entity, &mut SimParticle)>,
	timestep:		f32) {


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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="119">

---

This code snippet defines a function `reset_simulation_to_default` that resets simulation components to their default state. It deletes all particles, resets the grid, and resets the constraints. The `reset_simulation_to_default` function takes several arguments including `commands`, `constraints`, `grid`, and `particles`. The function prints a message to indicate that the simulation is being reset. The code also includes a struct `SimConstraints` that represents the simulation constraints and includes fields such as `grid_particle_ratio`, `timestep`, `incomp_iters_per_frame`, and others.

```renderscript
/// Reset simulation components to their default state and delete all particles.
pub fn reset_simulation_to_default(
	commands:			&mut Commands,
	mut constraints:	&mut SimConstraints,
	mut grid:			&mut SimGrid,
	particles:			&Query<(Entity, &mut SimParticle)>) {

	println!("Resetting simulation to default...");

	// Reset all particles.
	delete_all_particles(commands, constraints, grid, particles);

	// Reset the grid.
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

	// Reset constraints.
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

#[derive(Resource, Clone)]
pub struct SimConstraints {
	pub grid_particle_ratio:		f32, 	// PIC/FLIP simulation ratio (0.0 = FLIP, 1.0 = PIC).
	pub timestep:					f32,	// Timestep for simulation updates.
	pub incomp_iters_per_frame:		u8, 	// Simulation incompressibility iterations per frame.
	pub collision_iters_per_frame:	u8,		// Collision iterations per frame.
	pub gravity:					Vec2,	// Cartesian gravity vector.
	pub particle_radius:			f32,	// Particle collision radii.
	pub particle_count:				usize,	// Number of particles in the simulation.
	pub particle_rest_density:		f32,	// Rest density of particles in simulation.
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="168">

---

This code snippet is a default implementation of the `Default` trait for the `SimConstraints` struct. It defines default values for various simulation constraints such as `grid_particle_ratio`, `timestep`, `incomp_iters_per_frame`, `collision_iters_per_frame`, `gravity`, `particle_radius`, `particle_count`, and `particle_rest_density`.

```renderscript
impl Default for SimConstraints {

	fn default() -> SimConstraints {
		SimConstraints {
			grid_particle_ratio:		0.3,	// 0.0 = inviscid (FLIP), 1.0 = viscous (PIC).
			timestep:					1.0 / 120.0,
			incomp_iters_per_frame:		100,
			collision_iters_per_frame:	2,

			// (9.81^2) * 2 = ~385 (Bevy caps FPS at 60, we run sim at 120).
			gravity:					Vec2 { x: 0.0, y: -385.0 },
			particle_radius:			1.0,
			particle_count:				0,
			particle_rest_density:		0.0,
		}
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="186">

---

This code defines a struct `SimConstraints` with several methods to modify the constraints of a simulation. The `change_gravity` method changes the gravity direction and strength. The `toggle_simulation_pause` method toggles the simulation pause by changing the number of incomplete iterations per frame. The `change_incompressibility_timestep` method changes the number of iterations for incompressibility per frame. The `change_collision_timestep` method changes the number of iterations for particle collision per frame.

```renderscript
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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="221">

---

This code snippet defines a struct called `SimGrid` which represents a simulation grid. It contains fields such as `dimensions` (representing the number of horizontal and vertical cells in the simulation), `cell_size`, `cell_type` (a 2D vector of `SimGridCellType`), `cell_center` (a 2D vector of floating-point values representing the magnitude of pressure at the center of each cell), `velocity_u` and `velocity_v` (2D vectors representing horizontal and vertical magnitudes of velocity), `spatial_lookup` (a 2D vector of `Entity` objects), and `density` (a vector of floating-point values representing the density for each grid cell). The struct implements the `Default` trait to provide a default instance of the `SimGrid` struct with predefined values for its fields.

```renderscript
#[derive(Resource, Clone)]
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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="251">

---

This code snippet is a function `change_dimensions` that creates a new SimGrid with the provided dimensions and cell size. It initializes various arrays and vectors with default values based on the dimensions.

```renderscript
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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="267">

---

This code snippet sets the type of a cell in a simulation grid based on the provided x and y coordinates.

```renderscript
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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="286">

---

This code snippet sets the simulation grid dimensions by updating the `dimensions` field of the current object with the provided `width` and `height` values.

```renderscript
	/// Set simulation grid dimensions.
    pub fn set_grid_dimensions(
        &mut self,
        width: u16,
        height: u16) -> Result<()> {

        self.dimensions = (width, height);

        Ok(())
    }
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="297">

---

This code snippet sets the simulation grid cell size by updating the `cell_size` field of the struct with the provided `cell_size` value.

```renderscript
	// Set simulation grid cell size.
    pub fn set_grid_cell_size(
        &mut self,
        cell_size: u16) -> Result<()> {

        self.cell_size = cell_size;

        Ok(())
    }
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="307">

---

This code snippet calculates the position of a point in a grid based on its row and column indices, and whether it is a horizontal or vertical point. It takes into account the dimensions and cell size of the grid, as well as an offset. It returns a `Vec2` containing the x and y coordinates of the point.

```renderscript
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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="337">

---

This code snippet is a function named `get_cell_type_value` that returns the collision value of a cell. It checks if the `cell_row` and `cell_col` arguments are within bounds, and if not, it returns 0. If the cell type is `SimGridCellType::Solid`, it returns 0. If the cell type is `SimGridCellType::Fluid` or `SimGridCellType::Air`, it returns 1.

```renderscript
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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="357">

---

This code snippet converts a given position in a 2D grid to its corresponding coordinates (row, column). It also ensures that the resulting coordinates are within the bounds of the grid.

```renderscript
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

```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="377">

---

This code snippet is a function named `get_cell_position_from_coordinates` that takes in a `Vec2` coordinates and converts them into a position `Vec2`. It calculates the position based on the provided coordinates and grid size. If the input coordinates are invalid, it returns the closest valid position within the grid's bounds.

```renderscript
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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="398">

---

This code snippet calculates the center position of a cell on a grid based on its coordinates.

```renderscript
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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="412">

---

Kade/Garrett will add (Ai failed)

```renderscript
	/** Selects grid cells that entirely cover the a circle of radius `radius` centered at `position`;
		returns a Vector containing each cell's coordinates.  Note: the returned vector is of a
		static size.  If any cells in the selection are outside of the grid, then the closest valid
		cells will be added into the result.  **This can result in duplicated cell values, which is
		necessary to ensure accurate density calculations (corner cells would otherwise be
		considered much less dense than cells with selections entirely contained within the grid).** */
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

			let cell_coordinates = self.get_cell_coordinates_from_position(&cell_position);

			// Add our selected cell's coordinates to our list of cell coordinates!
			cells_in_selection[cell_index] = cell_coordinates;
		}

		cells_in_selection
	}

```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="477">

---

This code snippet clears all density values within a grid by setting them to 0.0.

```renderscript
	/// Set all density values within the grid to 0.0.
	pub fn clear_density_values(&mut self) {
		for density in self.density.iter_mut() {
			*density = 0.0;
		}
	}

```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="484">

---

This code snippet updates the density of each grid cell based on the weighted influences of particles. It selects nearby cells, calculates the center cell, and then iterates through each nearby cell to calculate the density weight based on the distance to the particle position. The density of each cell is then updated based on the density weight.

```renderscript
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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="509">

---

This code snippet calculates the density of a position within a grid by interpolating the densities of nearby cells based on their distances from the center cell.

```renderscript

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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="537">

---

This code snippet calculates the lookup index for a given cell coordinates in a spatial lookup table based on the table dimensions.

```renderscript
	// Get a cell lookup index into our spatial lookup table.
	pub fn get_lookup_index(&self, cell_coordinates: Vec2) -> usize {
		(cell_coordinates[1] as u16 + (cell_coordinates[0] as u16 * self.dimensions.0)) as usize
	}

```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="543">

---

This code snippet adds a new particle to a spatial lookup table.

```renderscript
	/// Add a new particle into our spatial lookup table.
	pub fn add_particle_to_lookup(&mut self, particle_id: Entity, lookup_index: usize) {

		if lookup_index > self.spatial_lookup.len() {
			eprintln!("Particle lookup index is out-of-bounds; cannot add particle to table!");
			return;
		}
		self.spatial_lookup[lookup_index].push(particle_id);
	}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="553">

---

This code snippet removes a particle from a spatial lookup table. It takes in a `particle_id` and a `lookup_index` as arguments. If the `lookup_index` is out of bounds, an error message is printed and the function returns. Otherwise, it searches through the spatial lookup at the specified `lookup_index` and removes the `particle_id` if it is found.

```renderscript
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

```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="572">

---

This code snippet defines a function `get_particles_in_lookup` that takes a `lookup_index` as an argument and returns a `Vec<Entity>` containing the particles currently inside the cell at the specified `lookup_index`. If the `lookup_index` is out of bounds, an empty vector is returned. The function iterates over the particles in the `spatial_lookup` at the `lookup_index`, skipping any particles with the value `Entity::PLACEHOLDER`, and adds the remaining particles to the `lookup_vector` which is then returned.

```renderscript
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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="595">

---

This code snippet is a function called `delete_all_particles_in_cell` that deletes all particles within a cell, specified by its lookup index. It takes in `commands`, `constraints`, `particles`, and `lookup_index` as arguments. It iterates through the particles in the spatial lookup table at the given index and despawns each particle entity using `commands.entity(*particle_id).despawn()`. It also decrements the `particle_count` in `constraints` if it is greater than 0. Finally, it clears the spatial lookup table at the current index.

```renderscript
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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="619">

---

This code snippet calculates the velocity of a cell. It takes the row and column indices of the cell as input, checks if the indices are within the dimensions of the grid and if they are not at the edges, and returns the velocity as a `Vec2` object.

```renderscript
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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="640">

---

This code snippet defines a function `get_nearby_particles` that takes a `lookup_index` as input and returns a vector of `Entity` objects. The function calculates the indices of the 9 cells surrounding the `lookup_index` and checks if they are valid. It then calls the `get_particles_in_lookup` function for each valid cell index and appends the resulting particles to the `nearby_particles` vector. Finally, it returns the `nearby_particles` vector.

```renderscript
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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="690">

---

This code snippet goes through the entire grid and labels the cells with their respective type.

```renderscript
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
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/mod.rs" line="732">

---

This code snippet defines a struct `SimParticle` with fields for position, velocity, and a lookup index. There are also two functions `test_setup` and `test_update` for initializing and updating the simulation state manager respectively. The `test_setup` function sets up the simulation layout by calling a helper function `construct_test_simulation_layout` with the `constraints`, `grid`, and `commands` as arguments. The `test_update` function updates the simulation by calling `step_simulation_once` with the `constraints`, `grid`, `particles`, and `fixed_timestep` as arguments.

```renderscript
#[derive(Component, Debug, Clone)]
pub struct SimParticle {
	pub position:		Vec2, 	// This particle's [x, y] position.
	pub velocity:		Vec2, 	// This particle's [x, y] velocity.
	pub lookup_index:	usize,	// Bucket index into spatial lookup for efficient neighbor search.
}

/// Simulation state manager initialization.
pub fn test_setup(
	commands:			Commands,
	mut constraints:	ResMut<SimConstraints>,
	mut grid:			ResMut<SimGrid>) {

	test_state_manager::construct_test_simulation_layout(
		constraints.as_mut(),
		grid.as_mut(),
		commands
	);

}

pub fn test_update(
	mut constraints:	ResMut<SimConstraints>,
	mut grid:			ResMut<SimGrid>,
	mut particles:		Query<(Entity, &mut SimParticle)>,
    ) {

	// let delta_time: f32 = time.delta().as_millis() as f32 * 0.001;
	let fixed_timestep: f32 = constraints.timestep;

    step_simulation_once(
        constraints.as_mut(),
        grid.as_mut(),
        &mut particles,
        fixed_timestep
    );

```

---

</SwmSnippet>

### <SwmPath>[src/simulation/sim_physics_engine.rs](/src/simulation/sim_physics_engine.rs)</SwmPath>

### <SwmPath>[src/simulation/sim_state_manager.rs](/src/simulation/sim_state_manager.rs)</SwmPath>

## Test

## <SwmPath>[src/juice_renderer.rs](/src/juice_renderer.rs)</SwmPath>

&nbsp;

<SwmMeta version="3.0.0" repo-id="Z2l0aHViJTNBJTNBSnVpY2VCb3glM0ElM0FBZXJvbmF1dGljYWwtQXhvbG90bHM=" repo-name="JuiceBox"><sup>Powered by [Swimm](https://app.swimm.io/)</sup></SwmMeta>
