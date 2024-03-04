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

Kade/Garret will fill in

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

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="8">

---

This code snippet applies particle velocities to grid velocity points. It calculates the influence of each particle on the nearby velocity points of the grid based on their distance from the points. The velocities are then added up and divided by the summation of all their influences. The code initializes new grids for velocity_u and velocity_v. It iterates through each horizontal u velocity point in the MAC grid and checks if it lies on the simulation boundary or between two air cells before applying the particle velocities.

```renderscript
/// Applies Particle velocities to grid velocity points
pub fn particles_to_grid(grid: &mut SimGrid, particles: &mut Query<(Entity, &mut SimParticle)>) -> SimGrid {

    // for velocity_u points and velocity_v points,
    // add up all particle velocities nearby scaled
    // by their distance / cell width (their influence)
    // then divide by the summation of all their
    // influences

    // This function, after applying particle velocities
    // to the grid, returns the previous grid

    // easy measurement for half the cell size
    let half_cell = grid.cell_size as f32 / 2.0;

    // Create new, blank grids
	let mut velocity_u = vec![vec![f32::MIN; (grid.dimensions.0 + 1) as usize]; grid.dimensions.1 as usize];
    let mut velocity_v = vec![vec![f32::MIN; grid.dimensions.0 as usize]; (grid.dimensions.1 + 1) as usize];

    // Go through each horizontal u velocity point in the MAC grid
    for row_index in 0..grid.dimensions.0 as usize {
        for col_index in 0..grid.dimensions.1 as usize + 1 {

            // Get (x, y) of current velocity point
            let pos = grid.get_velocity_point_pos(
                row_index,
                col_index,
                true);

            let left_center = pos - Vec2::new(half_cell, 0.0);
            let right_center = pos + Vec2::new(half_cell, 0.0);

            // If the velocity point lies on the simulation
            // boundary, skip it
            if left_center.x < 0.0 {
                continue;
            }

            if right_center.x > grid.dimensions.1 as f32 * grid.cell_size as f32 {
                continue;
            }

            // Determine if this velocity point lies between two air cells, and if so,
            // skip it
            let left_center_coords = grid.get_cell_coordinates_from_position(&left_center);
            let right_center_coords = grid.get_cell_coordinates_from_position(&right_center);

            if grid.cell_type[left_center_coords.x as usize][left_center_coords.y as usize] == SimGridCellType::Air && grid.cell_type[right_center_coords.x as usize][right_center_coords.y as usize] == SimGridCellType::Air {
                continue;
            }

            if grid.cell_type[left_center_coords.x as usize][left_center_coords.y as usize] == SimGridCellType::Solid && grid.cell_type[right_center_coords.x as usize][right_center_coords.y as usize] == SimGridCellType::Solid {
                continue;
            }

            let mut scaled_velocity_sum = 0.0;

            let mut scaled_influence_sum = 0.0;

            particles.for_each(|(_, particle)| {
                let influence = find_influence(
                    particle.position,
                    pos,
                    grid.cell_size);

                if influence != 0.0 {
                    scaled_influence_sum += influence;
                    scaled_velocity_sum += particle.velocity[0] * influence;
                }
            });

            if scaled_influence_sum == 0.0 {
                velocity_u[row_index][col_index] = 0.0;
                continue;
            }

            let new_velocity = scaled_velocity_sum / scaled_influence_sum;

            velocity_u[row_index][col_index] = new_velocity;
        }
    }

    // Do the same thing for vertical velocity points within the MAC grid
    for row_index in 0..grid.dimensions.0 as usize + 1 {
        for col_index in 0..grid.dimensions.1 as usize {

            let pos = grid.get_velocity_point_pos(
                row_index,
                col_index,
                false);

            let bottom_center = pos - Vec2::new(0.0, half_cell);
            let top_center = pos + Vec2::new(0.0, half_cell);

            if bottom_center.x < 0.0 {
                continue;
            }

            if top_center.x > grid.dimensions.0 as f32 * grid.cell_size as f32 {
                continue;
            }

            let bottom_center_coords = grid.get_cell_coordinates_from_position(&bottom_center);
            let top_center_coords = grid.get_cell_coordinates_from_position(&top_center);

            if grid.cell_type[bottom_center_coords.x as usize][bottom_center_coords.y as usize] == SimGridCellType::Air && grid.cell_type[top_center_coords.x as usize][top_center_coords.y as usize] == SimGridCellType::Air {
                continue;
            }

            if grid.cell_type[bottom_center_coords.x as usize][bottom_center_coords.y as usize] == SimGridCellType::Solid && grid.cell_type[top_center_coords.x as usize][top_center_coords.y as usize] == SimGridCellType::Solid {
                continue;
            }

            let mut scaled_velocity_sum = 0.0;

            let mut scaled_influence_sum = 0.0;

            particles.for_each(|(_, particle)| {
                let influence = find_influence(
                    particle.position,
                    pos,
                    grid.cell_size);

                if influence != 0.0 {
                    scaled_influence_sum += influence;
                    scaled_velocity_sum += particle.velocity[1] * influence;
                }

            });

            if scaled_influence_sum == 0.0 {
                velocity_v[row_index][col_index] = 0.0;
                continue;
            }

            let new_velocity = scaled_velocity_sum / scaled_influence_sum;

            velocity_v[row_index][col_index] = new_velocity;
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="8">

---

This code snippet applies particle velocities to grid velocity points. It creates new, blank grids for velocity_u and velocity_v. Then, it iterates over each horizontal u velocity point in the MAC grid and calculates the position of the left and right centers. If the velocity point lies on the simulation boundary or between two air cells, it is skipped. Finally, it returns the previous grid after applying particle velocities to the grid.

```renderscript
/// Applies Particle velocities to grid velocity points
pub fn particles_to_grid(grid: &mut SimGrid, particles: &mut Query<(Entity, &mut SimParticle)>) -> SimGrid {

    // for velocity_u points and velocity_v points,
    // add up all particle velocities nearby scaled
    // by their distance / cell width (their influence)
    // then divide by the summation of all their
    // influences

    // This function, after applying particle velocities
    // to the grid, returns the previous grid

    // easy measurement for half the cell size
    let half_cell = grid.cell_size as f32 / 2.0;

    // Create new, blank grids
	let mut velocity_u = vec![vec![f32::MIN; (grid.dimensions.0 + 1) as usize]; grid.dimensions.1 as usize];
    let mut velocity_v = vec![vec![f32::MIN; grid.dimensions.0 as usize]; (grid.dimensions.1 + 1) as usize];

    // Go through each horizontal u velocity point in the MAC grid
    for row_index in 0..grid.dimensions.0 as usize {
        for col_index in 0..grid.dimensions.1 as usize + 1 {

            // Get (x, y) of current velocity point
            let pos = grid.get_velocity_point_pos(
                row_index,
                col_index,
                true);

            let left_center = pos - Vec2::new(half_cell, 0.0);
            let right_center = pos + Vec2::new(half_cell, 0.0);

            // If the velocity point lies on the simulation
            // boundary, skip it
            if left_center.x < 0.0 {
                continue;
            }

            if right_center.x > grid.dimensions.1 as f32 * grid.cell_size as f32 {
                continue;
            }

            // Determine if this velocity point lies between two air cells, and if so,
            // skip it
            let left_center_coords = grid.get_cell_coordinates_from_position(&left_center);
            let right_center_coords = grid.get_cell_coordinates_from_position(&right_center);

            if grid.cell_type[left_center_coords.x as usize][left_center_coords.y as usize] == SimGridCellType::Air && grid.cell_type[right_center_coords.x as usize][right_center_coords.y as usize] == SimGridCellType::Air {
                continue;
            }

            if grid.cell_type[left_center_coords.x as usize][left_center_coords.y as usize] == SimGridCellType::Solid && grid.cell_type[right_center_coords.x as usize][right_center_coords.y as usize] == SimGridCellType::Solid {
                continue;
            }

            let mut scaled_velocity_sum = 0.0;

            let mut scaled_influence_sum = 0.0;

            particles.for_each(|(_, particle)| {
                let influence = find_influence(
                    particle.position,
                    pos,
                    grid.cell_size);

                if influence != 0.0 {
                    scaled_influence_sum += influence;
                    scaled_velocity_sum += particle.velocity[0] * influence;
                }
            });

            if scaled_influence_sum == 0.0 {
                velocity_u[row_index][col_index] = 0.0;
                continue;
            }

            let new_velocity = scaled_velocity_sum / scaled_influence_sum;

            velocity_u[row_index][col_index] = new_velocity;
        }
    }

    // Do the same thing for vertical velocity points within the MAC grid
    for row_index in 0..grid.dimensions.0 as usize + 1 {
        for col_index in 0..grid.dimensions.1 as usize {

            let pos = grid.get_velocity_point_pos(
                row_index,
                col_index,
                false);

            let bottom_center = pos - Vec2::new(0.0, half_cell);
            let top_center = pos + Vec2::new(0.0, half_cell);

            if bottom_center.x < 0.0 {
                continue;
            }

            if top_center.x > grid.dimensions.0 as f32 * grid.cell_size as f32 {
                continue;
            }

            let bottom_center_coords = grid.get_cell_coordinates_from_position(&bottom_center);
            let top_center_coords = grid.get_cell_coordinates_from_position(&top_center);

            if grid.cell_type[bottom_center_coords.x as usize][bottom_center_coords.y as usize] == SimGridCellType::Air && grid.cell_type[top_center_coords.x as usize][top_center_coords.y as usize] == SimGridCellType::Air {
                continue;
            }

            if grid.cell_type[bottom_center_coords.x as usize][bottom_center_coords.y as usize] == SimGridCellType::Solid && grid.cell_type[top_center_coords.x as usize][top_center_coords.y as usize] == SimGridCellType::Solid {
                continue;
            }

            let mut scaled_velocity_sum = 0.0;

            let mut scaled_influence_sum = 0.0;

            particles.for_each(|(_, particle)| {
                let influence = find_influence(
                    particle.position,
                    pos,
                    grid.cell_size);

                if influence != 0.0 {
                    scaled_influence_sum += influence;
                    scaled_velocity_sum += particle.velocity[1] * influence;
                }

            });

            if scaled_influence_sum == 0.0 {
                velocity_v[row_index][col_index] = 0.0;
                continue;
            }

            let new_velocity = scaled_velocity_sum / scaled_influence_sum;

            velocity_v[row_index][col_index] = new_velocity;
        }
    }

    let old_grid = grid.clone();

    grid.velocity_u = velocity_u;
    grid.velocity_v = velocity_v;

    old_grid

}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="158">

---

This code snippet creates a `SimGrid` that represents the difference between an old grid and a new grid. It calculates the change in velocity values (`velocity_u` and `velocity_v`) between the two grids and stores them in the `change_grid` object. The resulting `change_grid` is returned as the output of the function.

```renderscript
/**
    Create a SimGrid with values containing the difference between
    The old grid and new grid
*/
pub fn create_change_grid(old_grid: &SimGrid, new_grid: &SimGrid) -> SimGrid {

    // Here we are creating a SimGrid that holds the delta or change
    // in values after applying the particle velocities to the grid.
    // These values are needed when interpolating the velocity
    // values transfered to the particles from the grid.

    let (rows, cols) = old_grid.dimensions;

    let mut change_grid = old_grid.clone();
	let mut change_u = vec![vec![f32::MIN; (old_grid.dimensions.0 + 1) as usize]; old_grid.dimensions.1 as usize];
    let mut change_v = vec![vec![f32::MIN; old_grid.dimensions.0 as usize]; (old_grid.dimensions.1 + 1) as usize];

    for row_index in 0..rows as usize {
        for col_index in 0..(cols as usize + 1) {

            let change_in_u = new_grid.velocity_u[row_index][col_index] - old_grid.velocity_u[row_index][col_index];

            change_u[row_index][col_index] = change_in_u;
        }
    }

    for row_index in 0..(rows as usize + 1) {
        for col_index in 0..cols as usize {

            let change_in_v =  new_grid.velocity_v[row_index][col_index] - old_grid.velocity_v[row_index][col_index];

            change_v[row_index][col_index] = change_in_v;
        }
    }

    change_grid.velocity_u = change_u;
    change_grid.velocity_v = change_v;

    change_grid

}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="200">

---

This code snippet is a part of a function `extrapolate_values` which implements the Fast Sweeping algorithm. It extrapolates values in `velocity_u` and `velocity_v` up to the specified `depth`.

```renderscript
/**
    Extrapolates values in velocity_u and velocity_v up to the stated depth
    using the Fast Sweeping algorithm
*/

pub fn extrapolate_values(grid: &mut SimGrid, depth: i32) {

    let (rows, cols) = grid.dimensions;

    let mut d_u = vec![vec![0; (cols + 1) as usize]; rows as usize];
    let mut d_v = vec![vec![0; cols as usize]; (rows + 1) as usize];

    // Initialize caches for u and v components
    for row in 0..rows as usize {
        for col in 0..cols as usize + 1 {
            if grid.velocity_u[row][col] != f32::MIN {
                d_u[row][col] = 0;
            }
            else {
                d_u[row][col] = i32::MAX;
            }
        }
    }

    for row in 0..rows as usize + 1 {
        for col in 0..cols as usize {
            if grid.velocity_v[row][col] != f32::MIN {
                d_v[row][col] = 0;
            }
            else {
                d_v[row][col] = i32::MAX;
            }
        }
    }

    let mut wave_u: Vec<Vec2> = Vec::new();
    let mut wave_v: Vec<Vec2> = Vec::new();

    // Set up surrounding index offsets
    let surrounding = [
        [-1, 1],
        [-1, 0],
        [-1, -1],
        [0, 1],
        [0, -1],
        [1, 1],
        [1, 0],
        [1, -1],
    ];

    // Create first waves for u and v components
    for row in 0..rows as usize {
        for col in 0..cols as usize + 1 {
            if d_u[row][col] != 0 {
                if check_surrounding(&d_u, surrounding, (row, col), 0).len() != 0 {
                    d_u[row][col] = 1;
                    wave_u.push(Vec2::new(row as f32, col as f32));
                }
            }
        }
    }

    for row in 0..rows as usize + 1 {
        for col in 0..cols as usize {
            if d_v[row][col] != 0 {
                if check_surrounding(&d_v, surrounding, (row, col), 0).len() != 0 {
                    d_v[row][col] = 1;
                    wave_v.push(Vec2::new(row as f32, col as f32));
                }
            }
        }
    }

    // For both u and v components, extend their
    // velocities to empty neighbor velocity points
    let mut wavefronts_u: Vec<Vec<Vec2>> = Vec::new();
    wavefronts_u.push(wave_u);
    let mut wavefronts_v: Vec<Vec<Vec2>> = Vec::new();
    wavefronts_v.push(wave_v);

    let mut curr_wave_index = 0;

    while curr_wave_index < depth {
        let cur_wave = wavefronts_u.iter().nth(curr_wave_index as usize).unwrap();

        let mut next_wave = Vec::new();

        for i in 0..cur_wave.len() {

            let index = cur_wave.iter().nth(i).unwrap();

            let mut average = 0.0;
            let mut num_used = 0;

            for k in 0..8 {
                let offset_x = surrounding[k][0];
                let offset_y = surrounding[k][1];
                let neighbor_x = index.y as i32 + offset_x;
                let neighbor_y =  index.x as i32 + offset_y;

                if neighbor_x >= 0 && neighbor_x < grid.velocity_u[0].len() as i32 && neighbor_y >= 0 && neighbor_y < grid.velocity_u.len() as i32 {
                    if d_u[neighbor_y as usize][neighbor_x as usize] < d_u[index.x as usize][index.y as usize] {
                        average += grid.velocity_u[neighbor_y as usize][neighbor_x as usize];
                        num_used += 1;
                    }
                    else if d_u[neighbor_y as usize][neighbor_x as usize] == i32::MAX {
                        d_u[neighbor_y as usize][neighbor_x as usize] = d_u[index.x as usize][index.y as usize] + 1;
                        next_wave.push(Vec2::new(neighbor_y as f32, neighbor_x as f32));
                    }
                }

            }
            average /= num_used as f32;
            grid.velocity_u[index.x as usize][index.y as usize] = average;
        }

        wavefronts_u.push(next_wave);
        curr_wave_index += 1;
    }

    curr_wave_index = 0;

    while curr_wave_index < depth {
        let cur_wave = wavefronts_v.iter().nth(curr_wave_index as usize).unwrap();

        let mut next_wave = Vec::new();

        for i in 0..cur_wave.len() {

            let index = cur_wave.iter().nth(i).unwrap();

            let mut average = 0.0;
            let mut num_used = 0;

            for k in 0..8 {
                let offset_x = surrounding[k][0];
                let offset_y = surrounding[k][1];
                let neighbor_x = index.y as i32 + offset_x;
                let neighbor_y =  index.x as i32 + offset_y;

                if neighbor_x >= 0 && neighbor_x < grid.velocity_v[0].len() as i32 && neighbor_y >= 0 && neighbor_y < grid.velocity_v.len() as i32 {
                    if d_v[neighbor_y as usize][neighbor_x as usize] < d_v[index.x as usize][index.y as usize] {
                        average += grid.velocity_v[neighbor_y as usize][neighbor_x as usize];
                        num_used += 1;
                    }
                    else if d_v[neighbor_y as usize][neighbor_x as usize] == i32::MAX {
                        d_v[neighbor_y as usize][neighbor_x as usize] = d_v[index.x as usize][index.y as usize] + 1;
                        next_wave.push(Vec2::new(neighbor_y as f32, neighbor_x as f32));
                    }
                }

            }
            average /= num_used as f32;
            grid.velocity_v[index.x as usize][index.y as usize] = average;
        }

        wavefronts_v.push(next_wave);
        curr_wave_index += 1;
    }



}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="364">

---

This code snippet is a helper function called `check_surrounding` that takes in a `grid` of type `Vec<Vec<i32>>`, a `surroundings` array of size 8 containing pairs of `i32` values, an `index` tuple of type `(usize, usize)`, and a `value` of type `i32`. It checks the surrounding velocity points of a given `index` in the `grid` and returns a vector `valid_neighbors` containing the indices of the valid neighbors that have the same `value` in the `grid`.

```renderscript
/**
    Helper function to check surrounding velocity points
*/
fn check_surrounding(grid: &Vec<Vec<i32>>, surroundings: [[i32; 2]; 8], index: (usize, usize), value: i32) -> Vec<i32> {
    let mut valid_neighbors: Vec<i32> = Vec::new();
    let grid_width = grid[0].len() as i32;
    let grid_height = grid.len() as i32;

    for i in 0..8 {
        let offset_x = surroundings[i][0];
        let offset_y = surroundings[i][1];
        let neighbor_x = index.1 as i32 + offset_x;
        let neighbor_y =  index.0 as i32 + offset_y;

        if neighbor_x >= 0 && neighbor_x < grid_width && neighbor_y >= 0 && neighbor_y < grid_height {
            if grid[neighbor_y as usize][neighbor_x as usize] == value {
                valid_neighbors.push(i as i32);
            }
        }
    }

    valid_neighbors
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="388">

---

This code snippet defines a function `collect_particles` that takes a grid, a center position, and a list of particles as input. It collects all the particles within a cell specified by the center position and returns a vector of particles with their ID and data.

```renderscript
/**
    Collects all the particles within a cell and returns
    a vector of particles with their ID and data
*/
fn collect_particles<'a>(
        grid: &SimGrid,
        center: Vec2,
        particles: &'a mut Query<(Entity, &mut SimParticle)>
    ) -> Vec<(Entity, Mut<'a, SimParticle>)> {

    let mut particle_bag = Vec::new();

    let index = grid.get_lookup_index(center);

    let particle_ids = grid.get_particles_in_lookup(index);

    if particle_ids.len() == 0 {
        return Vec::new();
    }

    // Goes through all the particles and selects only
    // particles within the cell and adds them
    // to the bag
    particles.for_each_mut(|particle| {
        if particle_ids.contains(&particle.0) {
            particle_bag.push(particle);
        }
    });

    particle_bag

}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="421">

---

This code snippet applies grid-based interpolation to update the velocities of particles based on the given grid points and constraints. It calculates the interpolated velocity and change in velocity for each particle using the `interpolate_velocity` function. Then, it combines the interpolated velocity and the change in velocity using a coefficient (`pic_coef`) to obtain the new velocity. Finally, it updates the particle's velocity by adding the gravity and multiplying by the timestep.

```renderscript
/**
    Interpolates new particle velocities from grid points for a given
    set of particles.
*/
fn apply_grid<'a>(
        particles: Vec<(Entity, Mut<'a, SimParticle>)>,
        grid: &SimGrid,
        change_grid: &SimGrid,
        constraints: &SimConstraints,
    ) {

    // New velocity value using equation from section 7.6
    // in Fluid Simulation for Computer Graphics, Second Edition
    // (Bridson, Robert)

    let pic_coef = constraints.grid_particle_ratio;

    for (_, mut particle) in particles {

        let interp_vel = interpolate_velocity(particle.position, &grid);
        let change_vel = interpolate_velocity(particle.position, &change_grid);

        let pic_velocity = interp_vel;
        let flip_velocity =  particle.velocity + change_vel;
        let new_velocity = (pic_coef * pic_velocity) + ((1.0 - pic_coef) * flip_velocity);
        particle.velocity = new_velocity + (constraints.gravity * constraints.timestep);

    }

}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="452">

---

This code snippet iterates over each cell in a grid, identifies the particles within that cell, and applies a grid transformation to their velocities.

```renderscript
/// Apply grid velocities to particle velocities
pub fn grid_to_particles(
        grid: &mut SimGrid,
        change_grid: &SimGrid,
        particles: &mut Query<(Entity, &mut SimParticle)>,
        constraints: &SimConstraints,
    ) {

    // Basic idea right now is to go through each cell,
    // figure out which particles are 'within' that cell,
    // then apply the grid transformation

    for row_index in 0..grid.dimensions.1 as usize {
        for col_index in 0..grid.dimensions.0 as usize {

            // Skip over looking for particles where
            // they are not located
            match grid.cell_type[row_index][col_index] {
                SimGridCellType::Air => {
                    continue;
                }
                SimGridCellType::Solid => {
                    continue;
                },
                SimGridCellType::Fluid => (),
            }

            // Grab the center postition of the cell
            let coords = Vec2::new(row_index as f32, col_index as f32);

            // Grab all the particles within this specific cell
            let particles_in_cell = collect_particles(grid, coords, particles);

            // Solve for the new velocities of the particles
            apply_grid(particles_in_cell, grid, change_grid, constraints);
        }
    }
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="491">

---

This code snippet updates the `lookup_index` of a particle based on its position and updates the grid's lookup table accordingly.

```renderscript
/// Update the particle's lookup_index based on position, then update the grid's lookup table.
pub fn update_particle_lookup(particle_id: Entity, particle: &mut SimParticle, grid: &mut SimGrid) {

	// Find the cell that this particle belongs to and update our spatial lookup accordingly.
	let cell_coordinates: Vec2	= grid.get_cell_coordinates_from_position(&particle.position);
	let lookup_index: usize		= grid.get_lookup_index(cell_coordinates);

	// Remove the particle from its old lookup cell and place it here in its new one.
	if !grid.spatial_lookup[lookup_index].contains(&particle_id) {

		grid.remove_particle_from_lookup(particle_id, particle.lookup_index);
		grid.spatial_lookup[lookup_index].push(particle_id);
		particle.lookup_index = lookup_index;
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="507">

---

This code snippet updates the position, cell type, spatial lookup, and density value of particles in a simulation. It integrates the velocity into the position of each particle, handles collisions, updates the spatial lookup in a grid, and updates the density value of the cell the particle is in.

```renderscript
/** For each particle: integrate velocity into position, update cell type, update spatial lookup,
	and update density value of the cell the particle is in. */
pub fn update_particles(
	constraints:	&SimConstraints,
	particles:		&mut Query<(Entity, &mut SimParticle)>,
	grid:			&mut SimGrid,
	delta_time:		f32) {

	grid.clear_density_values();

	for (id, mut particle) in particles.iter_mut() {

		// Integrate the particles while handling collisions.
		let target_velocity: Vec2 = particle.velocity + constraints.gravity * delta_time;
		let target_position: Vec2 = particle.position + target_velocity * delta_time;
		integrate_particle_with_collisions(
			constraints,
			grid,
			particle.as_mut(),
			&target_position,
			&target_velocity
		);

		// Update the grid's spatial lookup based on this particle's position!
		update_particle_lookup(id, particle.as_mut(), grid);

		// Update the grid's density value for this current cell.
		grid.update_grid_density(particle.position);
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="538">

---

This code snippet is a function `integrate_particle_with_collisions` that calculates the maximum distance a particle can move before hitting a solid. It takes in `constraints`, `grid`, `particle`, `target_position`, and `target_velocity` as arguments. If the `target_position` is not inside a solid cell, the particle's position and velocity are updated with the `target_position` and `target_velocity` respectively. If the `target_position` is inside a solid cell, the code checks which direction the particle moved into the cell and adjusts the particle's position and velocity accordingly to avoid collision with the solid cell.

```renderscript
/// Find the maximum distance a particle can move before hitting a solid!
fn integrate_particle_with_collisions(
	constraints:		&SimConstraints,
	grid:				&SimGrid,
	particle:			&mut SimParticle,
	target_position:	&Vec2,
	target_velocity:	&Vec2) {

	let target_coordinates: Vec2	= grid.get_cell_coordinates_from_position(&target_position);
	let target_cell_type: u8		= grid.get_cell_type_value(
		target_coordinates.x as usize,
		target_coordinates.y as usize
	);

	// If the target position is not inside of a solid cell, move as normal.
	if target_cell_type != 0 {
		particle.position = *target_position;
		particle.velocity = *target_velocity;

		// While we are headed for a solid cell, collide with it!
	} else {
		let cell_center: Vec2 = grid.get_cell_center_position_from_coordinates(&target_coordinates);

		// Check which direction the particle moved into the cell from this frame.
		let cell_half_size: f32	= (grid.cell_size as f32) / 2.0;
		let cell_left: f32		= cell_center.x - cell_half_size;// - constraints.particle_radius;
		let cell_right: f32		= cell_center.x + cell_half_size;// + constraints.particle_radius;
		let cell_top: f32		= cell_center.y + cell_half_size;// + constraints.particle_radius;
		let cell_bottom: f32	= cell_center.y - cell_half_size;// - constraints.particle_radius;

		// Set a small collision tolerance so our particles don't get stuck to walls.
		let tolerance: f32		= 0.1;

		if particle.position.x <= cell_left && target_position.x >= cell_left {
			particle.position.x = cell_left - tolerance;
			particle.velocity.x = 0.0;
		} else if particle.position.x >= cell_right && target_position.x <= cell_right {
			particle.position.x = cell_right + tolerance;
			particle.velocity.x = 0.0;
		} else {
			particle.velocity.x = target_velocity.x;
			particle.position.x = target_position.x;
		}

		if particle.position.y <= cell_bottom && target_position.y >= cell_bottom {
			particle.position.y = cell_bottom - tolerance;
			particle.velocity.y = 0.0;
		} else if particle.position.y >= cell_top && target_position.y <= cell_top {
			particle.position.y = cell_top + tolerance;
			particle.velocity.y = 0.0;
		} else {
			particle.velocity.y = target_velocity.y;
			particle.position.y = target_position.y;
		}
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="595">

---

This code snippet handles particle collisions with the grid. It iterates over each particle and checks for collisions with the grid boundaries. If a collision is detected, it adjusts the particle's position and velocity to prevent it from escaping the grid.

```renderscript
/// Handle particle collisions with the grid.
pub fn handle_particle_grid_collisions(
	constraints:	&SimConstraints,
	grid:			&SimGrid,
	particles:		&mut Query<(Entity, &mut SimParticle)>) {

	for (_, mut particle) in particles.iter_mut() {

		// Don't let particles escape the grid!
		let grid_width: f32		= (grid.cell_size * grid.dimensions.0) as f32;
		let grid_height: f32	= (grid.cell_size * grid.dimensions.1) as f32;

		// Left/right collision checks.
		if particle.position.x < constraints.particle_radius {
			particle.position.x = constraints.particle_radius;
			particle.velocity.x = 0.0;
		} else if particle.position.x > grid_width - constraints.particle_radius {
			particle.position.x = grid_width - constraints.particle_radius;
			particle.velocity.x = 0.0;
		}

		// Up/down collision checks.
		if particle.position.y < constraints.particle_radius {
			particle.position.y = constraints.particle_radius;
			particle.velocity.y = 0.0;
		} else if particle.position.y > grid_height - constraints.particle_radius {
			particle.position.y = grid_height - constraints.particle_radius;
			particle.velocity.y = 0.0;
		}
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="627">

---

This code snippet defines a function `push_particles_apart` that is responsible for pushing particles apart to account for drift and grid cells with incorrect densities. It iterates over the grid cells and for each cell, it retrieves the nearby particles and possible collisions. Then, it iterates over each particle within a neighboring grid cell and checks for collisions with other particles within the same cell. If a collision is detected, it separates the particle pair using the `separate_particle_pair` function.

```renderscript
/** Push particles apart so that we account for drift and grid cells with incorrect densities.
	TODO: Improve collision solving speed between particles within cells.  Lots of particles in
	one cell leads to a large slowdown. */
pub fn push_particles_apart(
	constraints:	&SimConstraints,
	grid:			&SimGrid,
	particles:		&mut Query<(Entity, &mut SimParticle)>) {

	for i in 0..constraints.collision_iters_per_frame {

		// For each grid cell.
		for lookup_index in 0..grid.spatial_lookup.len() {

			// Create a vector of all particles in all of the surrounding cells.
			let nearby_particles: Vec<Entity>		= grid.get_nearby_particles(lookup_index);
			let possible_collisions: Vec<Entity>	= nearby_particles.clone();

			// For each particle within neighboring grid cell.
			for particle0_id in nearby_particles.iter() {

				// For each OTHER particle within this grid cell.
				for particle1_id in possible_collisions.iter() {

					// Don't process a collision between ourself!
					if particle0_id == particle1_id {
						continue;
					}

					// Get both particles involved in the collision.
					let particle_combo_result = particles.get_many_mut([
						*particle0_id,
						*particle1_id,
					]);
					let particle_combo = match particle_combo_result {
						Ok(particle_combo_result)	=> particle_combo_result,
						Err(_error)					=> {
							// eprintln!("Invalid particle combo; skipping!");
							continue;
						},
					};

					// Push both particles apart.
					separate_particle_pair(constraints, grid, particle_combo);
				}
			}
		}
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="676">

---

This code snippet is a helper function called `separate_particle_pair` that is used to push two particles apart if they are too close to each other. It takes in `constraints`, `grid`, and `particle_combo` as parameters. It calculates the distance between the particles and checks if they need to be separated based on their collision radii. If separation is required, it calculates the distance to separate the particles by and updates their positions and velocities using the `integrate_particle_with_collisions` function.

```renderscript
/// Helper function for push_particles_apart().
fn separate_particle_pair(
	constraints:		&SimConstraints,
	grid:				&SimGrid,
	mut particle_combo:	[(Entity, Mut<'_, SimParticle>); 2]) {

	// Collision radii used to find the particle pair's push force on each other.
	let collision_radius: f32			= constraints.particle_radius * 2.0;
	let collision_radius_squared: f32	= collision_radius * collision_radius;

	// Figure out if we even need to push the particles apart in the first place!
	let mut delta_position: Vec2 = Vec2 {
		x: particle_combo[0].1.position[0] - particle_combo[1].1.position[0],
		y: particle_combo[0].1.position[1] - particle_combo[1].1.position[1]
	};
	let distance_squared: f32	= (delta_position.x * delta_position.x) + (delta_position.y * delta_position.y);
	if distance_squared > collision_radius_squared || distance_squared <= 0.01 {
		return;
	}

	// Calculate the distance we need to separate the particles by.
	let distance: f32			= distance_squared.sqrt();
	let separation_scale: f32	= 0.5 * (collision_radius - distance) / distance;
	delta_position *= separation_scale;

	// Move the particles apart!
	let target_velocity0: Vec2 = particle_combo[0].1.velocity;
	let target_velocity1: Vec2 = particle_combo[1].1.velocity;

	let target_position0: Vec2 = particle_combo[0].1.position + delta_position;
	let target_position1: Vec2 = particle_combo[1].1.position - delta_position;

	integrate_particle_with_collisions(
		constraints,
		grid,
		particle_combo[0].1.as_mut(),
		&target_position0,
		&target_velocity0
	);
	integrate_particle_with_collisions(
		constraints,
		grid,
		particle_combo[1].1.as_mut(),
		&target_position1,
		&target_velocity1
	);
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="724">

---

This code snippet is a function that enforces velocity incompressibility for each grid cell within a simulation. It calculates the particle rest density for the simulation domain, and then uses the Gauss-Seidel method to adjust the grid velocities based on surrounding solid cells. It also calculates the inflow/outflow (divergence) for each fluid cell, and performs density calculations. The code snippet is written in Rust.

```renderscript
/** Force velocity incompressibility for each grid cell within the simulation.  Uses the
	Gauss-Seidel method. */
pub fn make_grid_velocities_incompressible(
	grid:			&mut SimGrid,
	constraints: 	&mut SimConstraints) {

	// Get the "particle rest density" for the simulation domain.
	let mut fluid_cell_count: f32		= 0.0;
	let mut density_sum: f32			= 0.0;

	for i in 0..grid.density.len() {
		density_sum			+= grid.density[i];
		fluid_cell_count	+= 1.0;
	}

	if fluid_cell_count > 0.0 {
		constraints.particle_rest_density = density_sum / fluid_cell_count;
	}

	// Allows the user to make the simulation go BRRRRRRR or brrr.
	for _ in 0..constraints.incomp_iters_per_frame {

		/* For each grid cell, calculate the inflow/outflow (divergence).  Then, find out how many
			surrounding cells are solid, then adjust grid velocities accordingly. */
		for row in 0..grid.dimensions.0 {
			for col in 0..grid.dimensions.1 {

				// Continue if we are not inside of a fluid cell.
				if grid.cell_type[row as usize][col as usize] != SimGridCellType::Fluid {
					continue;
				}

				// Calculate and sum the solid modifier for each surrounding cell.
				let solids: [u8; 5]	= calculate_cell_solids(&grid, row as usize, col as usize);
				let left_solid: u8	= solids[1];
				let right_solid: u8	= solids[2];
				let up_solid: u8	= solids[3];
				let down_solid: u8	= solids[4];

				let solids_sum: u8	= left_solid + right_solid + up_solid + down_solid;
				if solids_sum == 0 {
					continue;
				}

				// Determine the inflow/outflow of the current cell.
				let mut divergence: f32 = calculate_cell_divergence(
					&grid,
					row as usize,
					col as usize
				);

				// Density calculations.
				if constraints.particle_rest_density > 0.0 {
					let stiffness: f32			= 1.0;
					let cell_coordinates: Vec2	= Vec2 {
						x: row as f32,
						y: col as f32
					};
					let density: f32		= grid.density[grid.get_lookup_index(cell_coordinates)];
					let compression: f32	= density - constraints.particle_rest_density;
					if compression > 0.0 {
						divergence -= stiffness * compression;
					}
				}

				// Force incompressibility on this cell.
				let overrelaxation: f32	= 1.99;
				let momentum: f32		= overrelaxation * ((0.0 - divergence) / solids_sum as f32);

				grid.velocity_u[row as usize][col as usize]			-= momentum * left_solid as f32;
				grid.velocity_u[row as usize][(col + 1) as usize]	+= momentum * right_solid as f32;
				grid.velocity_v[row as usize][col as usize]			+= momentum * up_solid as f32;
				grid.velocity_v[(row + 1) as usize][col as usize]	-= momentum * down_solid as f32;
			}
		}
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="802">

---

This code snippet calculates the divergence of a grid cell in a fluid simulation. The divergence represents the difference between inflow and outflow in the cell. A negative divergence indicates too much inflow, while a positive divergence indicates too much outflow. The code retrieves the velocities of the neighboring cells and calculates the divergence by subtracting the left velocity from the right velocity and the up velocity from the down velocity.

```renderscript
/** Calculate the divergence (inflow/outflow) of a grid cell.  If this number is not zero, then
	the fluid must be made incompressible.  **A negative divergence indicates there is too much
	inflow, whereas a positive divergence indicates too much outflow.** */
fn calculate_cell_divergence(
	grid:			&SimGrid,
	cell_row:		usize,
	cell_col:		usize,
) -> f32 {

	/* Retrieve velocities for each face of the current cell.  Note: this will not go out of
		bounds of the velocity arrays; each array is guaranteed to have sufficient space allocated
		to index like this. */
	let left_velocity: f32	= grid.velocity_u[cell_row][cell_col];
	let right_velocity: f32	= grid.velocity_u[cell_row][cell_col + 1];
	let up_velocity: f32	= grid.velocity_v[cell_row][cell_col];
	let down_velocity: f32	= grid.velocity_v[cell_row + 1][cell_col];

	// BUG: The up and down flows may need to be reversed.
	let x_divergence: f32	= right_velocity - left_velocity;
	let y_divergence: f32	= up_velocity - down_velocity;
	let divergence: f32		= x_divergence + y_divergence;

	divergence
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_physics_engine.rs" line="827">

---

This code snippet calculates the cell solid modifiers for a given cell in a grid. It retrieves the cell type values for the center, left, right, up, and down cells and returns them as an array.

```renderscript
/** Returns the cell solid modifiers (0 for solid, 1 otherwise) for cells in the order of: center,
	left, right, up, down. **/
fn calculate_cell_solids(grid: &SimGrid, cell_row: usize, cell_col: usize) -> [u8; 5] {

	/* Calculate collision modifiers for each cell face.  Note that we must perform a wrapping
		subtraction to prevent an underflow for our usize types. */
	let collision_center: u8	= grid.get_cell_type_value(cell_row, cell_col);
	let collision_left: u8		= grid.get_cell_type_value(usize::wrapping_sub(cell_col, 1), cell_col);
	let collision_right: u8		= grid.get_cell_type_value(cell_row, cell_col + 1);
	let collision_up: u8		= grid.get_cell_type_value(cell_row, usize::wrapping_sub(cell_row, 1));
	let collision_down: u8		= grid.get_cell_type_value(cell_row + 1, cell_col);

	[collision_center, collision_left, collision_right, collision_up, collision_down]
}
```

---

</SwmSnippet>

### <SwmPath>[src/simulation/sim_state_manager.rs](/src/simulation/sim_state_manager.rs)</SwmPath>

<SwmSnippet path="/src/simulation/sim_state_manager.rs" line="12">

---

This code snippet adds particles into a simulation within a given radius. It creates a center particle and concentric rings of particles that evenly space themselves out to form a circle around the center particle. The number of particles and their positions are determined by the `particle_density`, `radius`, and `center_position` parameters.

```renderscript
/** Add many particles into the simulation within a radius.  Note that particle_density is
	the number of particles per unit radius. */
pub fn add_particles_in_radius(
	commands:			&mut Commands,
	constraints:		&mut SimConstraints,
	grid:				&mut SimGrid,
	particle_density:	f32,
	radius:				f32,
	center_position:	Vec2,
	velocity:			Vec2) {

	// Create center particle.
	let _center_particle = add_particle(commands, constraints, grid, center_position, velocity);

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
			let _particle = add_particle(commands, constraints, grid, particle_position, velocity);
		}
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_state_manager.rs" line="53">

---

This code snippet adds particles into the simulation. It checks if the given position is within the simulation grid's bounds. If not, it returns an error. It also checks if the cell at the given position is a solid cell, if so it returns an error. Then it creates a particle entity with the given position and velocity, and adds it to the 0-cell's lookup. Finally, it increments the particle count and links a sprite to the particle for rendering.

```renderscript
/// Add particles into the simulation.
pub fn add_particle(
	commands:		&mut Commands,
	constraints:	&mut SimConstraints,
	grid:			&mut SimGrid,
	position:		Vec2,
	velocity:		Vec2) -> Result<()> {

	// Don't allow the user to create particles out of the simulation grid's bounds!
	if position[0] < 0.0 || position[0] > (grid.dimensions.1 * grid.cell_size) as f32 {
		return Err(Error::OutOfGridBounds(
			"X-coordinate for particle creation is out of grid bounds!"
		));
	}
	if position[1] < 0.0 || position[1] > (grid.dimensions.0 * grid.cell_size) as f32 {
		return Err(Error::OutOfGridBounds(
			"Y-coordinate for particle creation is out of grid bounds!"
		));
	}
	// If the cell we are inside of is a solid, don't create the particle!
	let cell_coordinates: Vec2 = grid.get_cell_coordinates_from_position(&position);
	if matches!(
		grid.cell_type[cell_coordinates[0] as usize][cell_coordinates[1] as usize],
		SimGridCellType::Solid) {
		return Err(Error::InvalidCellParticleCreation("Chosen cell is solid!"));
	}

	// Add every particle to the 0-cell's lookup at first; we will sort this next frame.
	let lookup_index: usize	= 0;
	let particle: Entity	= commands.spawn(
		SimParticle {
			position:		position,
			velocity:		velocity,
			lookup_index:	lookup_index,
		}
	).id();
	grid.add_particle_to_lookup(particle, lookup_index);

	constraints.particle_count += 1;

	// IMPORTANT: Links a sprite to each particle for rendering.
	juice_renderer::link_particle_sprite(commands, particle);

	Ok(())
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_state_manager.rs" line="99">

---

This code snippet defines a function `delete_particle` that removes a particle with a given ID from a simulation. It takes in several arguments including `commands`, `constraints`, `particles`, `grid`, and `particle_id`. It first checks if the particle with the given ID exists in the `particles` query by calling `particles.get(particle_id)`. If it does, it removes the particle from the `grid` and despawns it by calling `grid.remove_particle_from_lookup(particle_id, particle.1.lookup_index)` and `commands.entity(particle_id).despawn()`, respectively. Then, it decreases the `particle_count` in `constraints` if it is greater than 0. If the particle with the given ID doesn't exist, it returns an error.

```renderscript
/// Remove a particle with ID particle_id from the simulation.
pub fn delete_particle(
	commands:		&mut Commands,
	constraints:	&mut SimConstraints,
	particles:		&Query<(Entity, &mut SimParticle)>,
	grid:			&mut SimGrid,
	particle_id:	Entity) -> Result<()> {

	// Look for the particle in our particles query.
	if let Ok(particle) = particles.get(particle_id) {

		// Remove particle from lookup table and despawn it.
		grid.remove_particle_from_lookup(particle_id, particle.1.lookup_index);
		commands.entity(particle_id).despawn();

		/* BUG: This overflowed once while testing, and I'm betting it's because I misuse
			Entity::PLACEHOLDER.  Here is my silly little fix: */
		if constraints.particle_count > 0 {
			constraints.particle_count -= 1;
		}

		return Ok(());
	}

	Err(Error::InvalidEntityID("Invalid particle entity ID!"))
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_state_manager.rs" line="126">

---

This code snippet deletes all particles in a simulation by iterating over each particle and calling the `delete_particle` function.

```renderscript
/// Reset all simulation components to their default state.
pub fn delete_all_particles(
	commands:		&mut Commands,
	constraints:	&mut SimConstraints,
	grid:			&mut SimGrid,
	particles:		&Query<(Entity, &mut SimParticle)>) {
	
	// KILL THEM ALL!!!
	for (particle_id, _) in particles.iter() {
		let _ = delete_particle(commands, constraints, particles, grid, particle_id);
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/simulation/sim_state_manager.rs" line="139">

---

This code snippet selects particles within a circle centered at `position` with a given `radius` by iterating over a grid and checking the distance between each particle's position and the center of the circle. It returns a vector of entity IDs for the selected particles.

```renderscript
/** Returns a vector of entity ID's of each particle within a circle centered at `position` with
	radius `radius`; returns an empty vector if no particles are found. */
pub fn select_particles<'a>(
	particles:	&Query<(Entity, &mut SimParticle)>,
	grid:		&SimGrid,
	position:	Vec2,
	radius:		f32) -> Vec<Entity> {

	let mut selected_particles: Vec<Entity>	= Vec::new();

	// TODO: Maybe use map() here?  Idk.  Garrett I need u to explain map() to me I don't get it :(
	let selected_cell_coordinates: Vec<Vec2> = grid.select_grid_cells(position, radius);

	for i in 0..selected_cell_coordinates.len() {

		let cell_lookup_index: usize = grid.get_lookup_index(selected_cell_coordinates[i]);
		for particle_id in grid.get_particles_in_lookup(cell_lookup_index).iter() {

			// TODO: Error checking here.  Don't use unwrap() in production!
			let particle: &SimParticle = particles.get(*particle_id).unwrap().1;

			// Avoid an unnecessary sqrt() here:
			let distance: f32 = Vec2::distance_squared(position, particle.position);

			// If we are within our radius, add the particle to the list and return it!
			if distance < (radius * radius) {
				selected_particles.push(*particle_id);
			}
		}
	}

	selected_particles
}
```

---

</SwmSnippet>

## Test

## <SwmPath>[src/juice_renderer.rs](/src/juice_renderer.rs)</SwmPath>

<SwmSnippet path="/src/juice_renderer.rs" line="1">

---

This code imports various modules from the `bevy` crate and the `crate` module. It also imports a specific function `test_draw_gravity_vector_arrow` from the `test_renderer` module. Additionally, it imports some specific elements from the `util` module. The purpose of this code is not clear without further context.

```renderscript
use bevy::{
	core_pipeline::prelude::ClearColor, prelude::*, sprite::MaterialMesh2dBundle
};
use crate::{
	simulation::{
		SimConstraints,
		SimGrid,
		SimGridCellType,
		SimParticle,
	}, test::test_renderer::test_draw_gravity_vector_arrow, util::{
		self,
		JUICE_BLUE,
		JUICE_GREEN
	}
};
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="38">

---

This code snippet defines two enums: `FluidColorRenderType` and `FluidGridVectorType`. The `FluidColorRenderType` enum represents different types of rendering for fluid colors, such as `Arbitrary`, `Velocity`, `Pressure`, `Density`, `GridCell`, and `Spume`. The `FluidGridVectorType` enum represents different types of vector values for fluid grids, such as `Velocity` and `Gravity`.

```renderscript
enum FluidColorRenderType	{ Arbitrary, Velocity, Pressure, Density, GridCell, Spume }
enum FluidGridVectorType	{ Velocity, Gravity }

```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="41">

---

This code snippet defines a `FluidRenderData` struct and implements the `Default` trait for it. The struct has several fields including `color_render_type`, `arbitrary_color`, `velocity_magnitude_color_scale`, `pressure_magnitude_color_scale`, `density_magnitude_color_scale`, and `particle_render_scale`. The `Default` implementation sets default values for these fields.

```renderscript
#[derive(Resource)]
struct FluidRenderData {
	color_render_type:	FluidColorRenderType,
	arbitrary_color:	Color,
	velocity_magnitude_color_scale:	f32,
	pressure_magnitude_color_scale:	f32,
	density_magnitude_color_scale:	f32,
	particle_render_scale: f32
}

impl Default for FluidRenderData {

	fn default() -> Self {
		Self {
			color_render_type:	FluidColorRenderType::Velocity,
			arbitrary_color:	util::JUICE_YELLOW,
			velocity_magnitude_color_scale:	200.0,
			pressure_magnitude_color_scale:	100.0,
			density_magnitude_color_scale: 	100.0,
			particle_render_scale: 1.0,
		}
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="65">

---

This code snippet defines a `GridRenderData` struct that represents the data used for rendering a grid. It includes various properties such as `draw_grid`, `grid_color`, `solid_cell_color`, `draw_vectors`, `vector_type`, `vector_color`, and `vector_magnitude_scale`. The `Default` trait is implemented for this struct, providing a default set of values for its properties.

```renderscript
#[derive(Resource)]
struct GridRenderData {
	draw_grid:			bool,
	grid_color:			Color,
	solid_cell_color:	Color,

	draw_vectors:			bool,
	vector_type:			FluidGridVectorType,
	vector_color:			Color,
	vector_magnitude_scale:	f32,
}

impl Default for GridRenderData {

	fn default() -> Self {
		Self {
			draw_grid:			false,
			grid_color:			Color::DARK_GRAY,
			solid_cell_color:	Color::GOLD,

			draw_vectors:			false,
			vector_type:			FluidGridVectorType::Velocity,
			vector_color:			Color::WHITE,
			vector_magnitude_scale:	0.05,
		}
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="93">

---

This code snippet toggles the `draw_grid` boolean value of the `grid` object. If `draw_grid` is `true`, it sets it to `false`, otherwise, it sets it to `true`.

```renderscript
// toggle draw grid
fn toggle_draw_grid(grid: &mut GridRenderData) {
	if grid.draw_grid == true {
		grid.draw_grid = false;
	}
	else{
		grid.draw_grid = true;
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="103">

---

This code snippet toggles the `draw_vectors` property of a `grid` by checking its current value and updating it accordingly.

```renderscript
// toggle draw grid vectors
fn toggle_draw_grid_vectors(grid: &mut GridRenderData) {
	if grid.draw_vectors == true {
		grid.draw_vectors = false;
	}
	else{
		grid.draw_vectors = true;
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="113">

---

This code snippet sets up a rendering pipeline for a simulation. It spawns a camera at the center of the simulation world with a specific translation and rotation.

```renderscript
/// Custom rendering pipeline initialization.
fn setup_renderer(
	mut commands:	Commands,
	grid:			Res<SimGrid>,
	mut meshes:		ResMut<Assets<Mesh>>,
	mut materials:	ResMut<Assets<ColorMaterial>>) {

	// Spawn a camera to view our simulation world!
	commands.spawn(Camera2dBundle {
		transform: Transform {
			translation:	Vec3 {
				x: ((grid.dimensions.0 * grid.cell_size) as f32) / 2.0,
				y: ((grid.dimensions.1 * grid.cell_size) as f32) / 2.0,
				z: 0.0,
			},
			rotation:		Quat::IDENTITY,
			scale:			Vec3::ONE,
		},
		..default()
	});
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="135">

---

This code snippet creates and links a new sprite to the specified particle entity in the simulation.

```renderscript
/** Creates and links a new sprite to the specified particle; **Must be called each time a new
	particle is added to the simulation!** */
pub fn link_particle_sprite(mut commands: &mut Commands, particle: Entity) {
	commands.entity(particle).insert(SpriteBundle::default());
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="141">

---

&nbsp;

```renderscript
/// Update the visual transform of all particles to be rendered.
fn update_particle_position(
	constraints: Res<SimConstraints>,
	mut particles: Query<(&SimParticle, &mut Transform)>) {

	for (particle, mut transform) in particles.iter_mut() {
		transform.translation = Vec3 {
			x: particle.position.x,
			y: particle.position.y,
			/* IMPORTANT: Keep this at the same z-value for all particles.  This allows Bevy to do
				sprite batching, cutting render costs by quite a bit.  If we change the z-index we
				will likely see a large performance drop. */
			z: 0.0,
		};
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="158">

---

This code snippet updates the size of all particles to be rendered. It iterates over a collection of particles and multiplies the particle radius by 2 and the particle render scale to calculate the new size. Then, it sets the `custom_size` property of each particle's `Sprite` component to the calculated size.

```renderscript
/// Update the size of all particles to be rendered.
fn update_particle_size(
	mut particles:		Query<(&SimParticle, &mut Sprite)>,
	constraints:		Res<SimConstraints>,
	fluid_render_data:	Res<FluidRenderData>) {

	for (_, mut sprite) in particles.iter_mut() {
		/* Multiply this by 2, because we are dealing with the radius.  To account for the full
			size of the particle, we need to multiply the radius by 2. */
		let size: f32 = constraints.particle_radius * 2.0 * fluid_render_data.particle_render_scale;
		sprite.custom_size = Some(Vec2::splat(size));
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="172">

---

This code snippet updates the color of all particles to be rendered based on different criteria such as velocity, pressure, density, spume, arbitrary color, or grid cell.

```renderscript
/// Update the color of all particles to be rendered.
fn update_particle_color(
	particles:				Query<(&SimParticle, &mut Sprite)>,
	grid:					Res<SimGrid>,
	constraints:			Res<SimConstraints>,
	particle_render_data:	Res<FluidRenderData>) {

	match particle_render_data.color_render_type {
		FluidColorRenderType::Velocity	=> color_particles_by_velocity(
			particles,
			particle_render_data.velocity_magnitude_color_scale,
			&vec![util::JUICE_BLUE, util::JUICE_GREEN, util::JUICE_YELLOW, util::JUICE_RED]
		),
		FluidColorRenderType::Pressure	=> color_particles_by_pressure(
			particles,
			grid.as_ref(),
			particle_render_data.pressure_magnitude_color_scale,
			&vec![util::JUICE_BLUE, util::JUICE_GREEN, util::JUICE_YELLOW, util::JUICE_RED]
		),
		FluidColorRenderType::Density	=> color_particles_by_density(
			particles,
			grid.as_ref(),
			particle_render_data.density_magnitude_color_scale * constraints.particle_rest_density / constraints.particle_radius,
			&vec![util::JUICE_BLUE, util::JUICE_GREEN, util::JUICE_YELLOW, util::JUICE_RED]
		),
		FluidColorRenderType::Spume		=> color_particles_by_density(
			particles,
			grid.as_ref(),
			particle_render_data.density_magnitude_color_scale * constraints.particle_rest_density / constraints.particle_radius,
			&vec![Color::ANTIQUE_WHITE, util::JUICE_SKY_BLUE, util::JUICE_BLUE, util::JUICE_BLUE]
		),
		FluidColorRenderType::Arbitrary	=> color_particles(
			particles,
			particle_render_data.arbitrary_color
		),
		FluidColorRenderType::GridCell	=> color_particles_by_grid_cell(
			particles,
			grid.as_ref(),
			JUICE_BLUE,
			JUICE_GREEN
		),
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="216">

---

This code snippet colors all particles in a simulation based on their velocities. It takes in a query of particles and their sprites, a color scale for the velocity magnitude, and a list of colors. It iterates through each particle and calculates a color based on the velocity magnitude. Finally, it assigns the calculated color to the particle's sprite.

```renderscript
/// Color all particles in the simulation by their velocities.
fn color_particles_by_velocity(
	mut particles:					Query<(&SimParticle, &mut Sprite)>,
	velocity_magnitude_color_scale:	f32,
	color_list:						&Vec<Color>) {

	for (particle, mut sprite) in particles.iter_mut() {

		let color: Color = util::generate_color_from_gradient(
			color_list,
			util::vector_magnitude(particle.velocity) / velocity_magnitude_color_scale,
		);

		sprite.color = color;
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="233">

---

This code snippet colors all particles in the simulation based on their pressures. It takes in a `Query` of particles and their corresponding sprites, a `SimGrid`, a pressure magnitude color scale, and a list of colors. It iterates over each particle and sprite pair, gets the cell position of the particle, calculates the cell row and column, generates a color based on the pressure magnitude of the cell, and assigns the color to the sprite's `color` property.

```renderscript
/// Color all particles in the simulation by their pressures.
fn color_particles_by_pressure(
	mut particles:					Query<(&SimParticle, &mut Sprite)>,
	grid:							&SimGrid,
	pressure_magnitude_color_scale:	f32,
	color_list:						&Vec<Color>) {

	for (particle, mut sprite) in particles.iter_mut() {

		let cell_pos: Vec2	= grid.get_cell_coordinates_from_position(&particle.position);
		let cell_row: usize	= cell_pos[1] as usize;
		let cell_col: usize	= cell_pos[0] as usize;

		let color: Color = util::generate_color_from_gradient(
			color_list,
			grid.cell_center[cell_row][cell_col] / pressure_magnitude_color_scale,
		);
		sprite.color = color;
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="254">

---

This code snippet colors all particles in a simulation based on the density of the cell they belong to. It uses a gradient color scale and a density magnitude color scale to determine the color of each particle. The `color_particles_by_density` function takes in a list of particles, a simulation grid, a color scale, and a list of colors as arguments. It then iterates over each particle, calculates the density at its position using the `get_density_at_position` function from the grid, generates a color based on the density using the `generate_color_from_gradient` function, and assigns the color to the particle's sprite.

```renderscript
/// Color all particles in the simulation by the density of the cell they belong to.
fn color_particles_by_density(
	mut particles:					Query<(&SimParticle, &mut Sprite)>,
	grid:							&SimGrid,
	density_magnitude_color_scale:	f32,
	color_list:						&Vec<Color>) {

	for (particle, mut sprite) in particles.iter_mut() {

		let density: f32 = grid.get_density_at_position(particle.position);
		let color: Color = util::generate_color_from_gradient(
			color_list,
			density / (density_magnitude_color_scale * 0.45),
		);
		sprite.color = color;
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="272">

---

This code snippet defines a function `color_particles` that takes in a query of particles and a color as input. It iterates over all the particles in the query and sets their sprite color to the specified color.

```renderscript
/// Color all particles in the simulation as anything you want!
fn color_particles(mut particles: Query<(&SimParticle, &mut Sprite)>, color: Color) {

	for (_, mut sprite) in particles.iter_mut() {
		sprite.color = color;
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="280">

---

This code snippet colors all particles in the simulation based on their grid cell. It takes in two colors, `color_even` and `color_odd`, and assigns the respective color to each particle's `sprite` based on the sum of their cell row and cell column being even or odd.

```renderscript
/// Color all particles in the simulation by their grid cell.
fn color_particles_by_grid_cell(
	mut particles:	Query<(&SimParticle, &mut Sprite)>,
	grid:			&SimGrid,
	color_even:		Color,
	color_odd:		Color) {

	for (particle, mut sprite) in particles.iter_mut() {

		let cell_pos: Vec2	= grid.get_cell_coordinates_from_position(&particle.position);
		let cell_row: usize	= cell_pos[1] as usize;
		let cell_col: usize	= cell_pos[0] as usize;

		if (cell_row + cell_col) % 2 == 0 {
			sprite.color = color_even;
		} else {
			sprite.color = color_odd;
		}
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="301">

---

This code snippet is a function that draws the solid grid cells within a grid. It iterates through each cell in the grid and checks its type. If the cell is fluid or air, it does nothing. If the cell is solid, it calls a `draw_solid_cell` function to draw the cell using the provided grid, position, color, and `gizmos` object.

```renderscript
/// Draw the solid grid cells within the grid.
fn draw_grid_solids(grid: Res<SimGrid>, grid_render_data: Res<GridRenderData>, mut gizmos: Gizmos) {

	// For each column in each row, determine each cell's type.
	for row in 0..grid.dimensions.0 {
		for col in 0..grid.dimensions.1 {

			match grid.cell_type[row as usize][col as usize] {
				SimGridCellType::Fluid	=> continue,			// Do nothing if fluid.
				SimGridCellType::Air	=> continue,			// Do nothing if air.
				SimGridCellType::Solid	=> draw_solid_cell(		// Draw something if solid.
					grid.as_ref(),
					Vec2 { x: row as f32, y: col as f32 },
					grid_render_data.solid_cell_color,
					&mut gizmos
				),
			}
		}
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="322">

---

This code snippet draws a solid grid cell at a specific `cell_coordinates` (row, column) on a `grid` using the provided `color`. It uses the `draw_solid_cell` function to calculate the position of the cell and then draws a rectangle with the specified color at that position using the `gizmos` object.

```renderscript
/// Draw a solid grid cell using cell_coordinates (row, column).
fn draw_solid_cell(grid: &SimGrid, cell_coordinates: Vec2, color: Color, gizmos: &mut Gizmos) {

	// Get cell position.
	let grid_height: f32	= (grid.dimensions.1 * grid.cell_size) as f32;
	let half_cell_size: f32	= grid.cell_size as f32 * 0.5;
	let position: Vec2 = Vec2 {
		x: cell_coordinates[1] * (grid.cell_size as f32) + half_cell_size,
		y: grid_height - cell_coordinates[0] * (grid.cell_size as f32) - half_cell_size,
	};

	// Draw the cell.
	gizmos.rect_2d(
		position,
		0.0,
		Vec2::splat(grid.cell_size as f32),
		color
	);
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="342">

---

This code snippet draws grid cells based on a provided `SimGrid` using Bevy's Gizmos library. It first checks if `grid_render_data.draw_grid` is `false`. If so, it outlines the simulation by drawing lines around it. Otherwise, it draws vertical and horizontal grid lines using `gizmos.line_2d` function.

```renderscript
/// Draw grid cells based on SimGrid using Bevy's Gizmos!
fn draw_grid_cells(grid: Res<SimGrid>, grid_render_data: Res<GridRenderData>, mut gizmos: Gizmos) {

	let grid_width: f32		= (grid.dimensions.0 * grid.cell_size) as f32;
	let grid_height: f32	= (grid.dimensions.1 * grid.cell_size) as f32;

	// If we don't want to draw the grid cells, still outline the simulation.
	if !grid_render_data.draw_grid {

		let top_left: Vec2		= Vec2 { x: 0.0,		y: grid_height };
		let top_right: Vec2		= Vec2 { x: grid_width,	y: grid_height };
		let bottom_right: Vec2	= Vec2 { x: grid_width,	y: 0.0 };

		gizmos.line_2d(Vec2::ZERO, bottom_right, grid_render_data.grid_color);
		gizmos.line_2d(Vec2::ZERO, top_left, grid_render_data.grid_color);
		gizmos.line_2d(top_left, top_right, grid_render_data.grid_color);
		gizmos.line_2d(top_right, bottom_right, grid_render_data.grid_color);

		return;
	}

	// Draw vertical grid lines.
	for i in 0..((grid.dimensions.0 as u16) + 1) {
		let cell_bottom_position: Vec2 = Vec2 {
			x: (i * grid.cell_size) as f32,
			y: 0.0,
		};
		let cell_top_position: Vec2 = Vec2 {
			x: (i * grid.cell_size) as f32,
			y: grid_height,
		};
		gizmos.line_2d(cell_bottom_position, cell_top_position, grid_render_data.grid_color);
	}

	// Draw horizontal grid lines.
	for i in 0..((grid.dimensions.1 as u16) + 1) {
		let cell_left_position: Vec2 = Vec2 {
			x: 0.0,
			y: (i * grid.cell_size) as f32,
		};
		let cell_right_position: Vec2 = Vec2 {
			x: grid_width,
			y: (i * grid.cell_size) as f32,
		};
		gizmos.line_2d(cell_left_position, cell_right_position, grid_render_data.grid_color);
	}
}

```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="390">

---

This code snippet is a function called `draw_grid_vectors` that draws velocity vectors based on a `SimGrid` using Bevy's Gizmos. It takes the `grid` and `grid_render_data` resources as input parameters and modifies the `gizmos` resource. It iterates over each cell in the grid and calculates the horizontal and vertical velocity components. It then calculates the magnitude and direction of the velocity vector based on these components. If the magnitude is below a certain threshold, the vector is skipped. Finally, it calculates the center position of each grid cell and draws the vector arrows using the Gizmos API.

```renderscript
/// Draw velocity vectors based on SimGrid using Bevy's Gizmos!
fn draw_grid_vectors(
	grid:				Res<SimGrid>,
	grid_render_data:	Res<GridRenderData>,
	mut gizmos:			Gizmos) {

	if !grid_render_data.draw_vectors {
		return;
	}

	for row in 0..grid.dimensions.1 {
		for col in 0..grid.dimensions.0 {

			/* Indices for each column/row of each u/v velocity component on the grid.  Note that
				because each cell has two velocity components going in either direction, the
				vectors containing said components are one element larger in either rows or
				columns.  This fact prevents the following code from going out of bounds, so long
				as grid.velocity_u and grid.velocity_v are constructed properly. */
			let column_u0: usize	= col as usize;
			let column_u1: usize	= (col + 1) as usize;
			let row_u: usize		= row as usize;

			let row_v0: usize		= row as usize;
			let row_v1: usize		= (row + 1) as usize;
			let column_v: usize		= col as usize;

			// Horizontal velocity components.
			let velocities_u: [f32; 2]	= [
				grid.velocity_u[row_u][column_u0],
				grid.velocity_u[row_u][column_u1],
			];
			// Vertical velocity components.
			let velocities_v: [f32; 2]	= [
				grid.velocity_v[row_v0][column_v],
				grid.velocity_v[row_v1][column_v],
			];

			// Calculate magnitude of velocity within the call.
			let velocity_vector_cartesian: Vec2 = Vec2 {
				x: (velocities_u[0] + velocities_u[1]) / 2.0,
				y: (velocities_v[0] + velocities_v[1]) / 2.0,
			};

			// Calculate velocity direction and magnitude based on u and v components.
			let velocity_vector_polar: Vec2 = util::cartesian_to_polar(velocity_vector_cartesian);

			// Skip drawing if the vector is too short.
			if velocity_vector_polar[0] < 0.2 {
				continue;
			}

			// Find the center of each grid cell to draw the vector arrows.
			// TODO: Refactor to use grid.get_cell_center_position_from_coordinates().
			let half_cell_size: f32	= (grid.cell_size as f32) / 2.0;
			let cell_x: f32			= (col * grid.cell_size) as f32;
			let cell_y: f32			= (row * grid.cell_size) as f32;
			let grid_height: f32	= (grid.dimensions.0 * grid.cell_size) as f32;
			let cell_center_position: Vec2 = Vec2 {
				x: cell_x + half_cell_size,
				y: grid_height - cell_y - half_cell_size,
			};

			draw_vector_arrow(
				cell_center_position,
				velocity_vector_polar[1],
				velocity_vector_polar[0] * grid_render_data.vector_magnitude_scale,
				grid_render_data.vector_color,
				&mut gizmos
			);
		}
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="463">

---

This code snippet is a helper function that draws a vector arrow using Bevy's Gizmos. It takes in parameters such as the tail position, direction in radians, magnitude, arrow color, and the gizmos object. It constructs the main ray of the arrow, determines the size and angle of the arrow head, and constructs the left and right sides of the arrow. Finally, it uses the gizmos object to draw the arrow lines using the provided color.

```renderscript
/// Helper function to draw a vector arrow using Bevy's Gizmos.
pub fn draw_vector_arrow(
	tail_position:		Vec2,
	direction_rads:		f32,
	magnitude:			f32,
	arrow_color:		Color,
	gizmos:				&mut Gizmos) {

	// Construct main ray of arrow.
	let head_position: Vec2	= Vec2 {
		x: tail_position.x + direction_rads.cos() * magnitude,
		y: tail_position.y + direction_rads.sin() * magnitude,
	};

	// Grow or shrink the arrow head's angle depending on the magnitude (for aesthetic purposes).
	let arrow_angle_offset_rads: f32	= 0.61 - (magnitude / 1000.0);
	// Controls how large the arrow heads are relative to the arrow's body.
	let arrow_scale_ratio: f32			= 0.25 * magnitude;

	// Construct left side of arrow.
	let arrow_left_position: Vec2 = Vec2 {
		x: head_position.x - ((direction_rads - arrow_angle_offset_rads).cos() * arrow_scale_ratio),
		y: head_position.y - ((direction_rads - arrow_angle_offset_rads).sin() * arrow_scale_ratio),
	};

	// Construct right side of arrow.
	let arrow_right_position: Vec2 = Vec2 {
		x: head_position.x - ((direction_rads + arrow_angle_offset_rads).cos() * arrow_scale_ratio),
		y: head_position.y - ((direction_rads + arrow_angle_offset_rads).sin() * arrow_scale_ratio),
	};

	// Draw arrows!
	gizmos.line_2d(tail_position, head_position,		arrow_color);
	gizmos.line_2d(head_position, arrow_left_position,	arrow_color);
	gizmos.line_2d(head_position, arrow_right_position,	arrow_color);
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/juice_renderer.rs" line="500">

---

This code snippet draws a circle around the mouse cursor using the `circle_2d` function from the `Gizmos` module. It takes the `position`, `radius`, and `color` as parameters.

```renderscript
/// Draws a circle around the mouse cursor.
pub fn draw_selection_circle(gizmos: &mut Gizmos, position: Vec2, radius: f32, color: Color) {
	gizmos.circle_2d(position, radius, color);
}
```

---

</SwmSnippet>

## <SwmPath>[src/ui.rs](/src/ui.rs)</SwmPath>

<SwmSnippet path="/src/ui.rs" line="1">

---

&nbsp;

```renderscript
use std::mem::transmute;

use bevy::{asset::{AssetServer, Assets, Handle}, ecs::system::{Query, Res, ResMut, Resource}, prelude::default, render::{color::Color, texture::Image}, ui::FlexWrap, window::Window};
use bevy_egui::{egui::{self, color_picker::color_edit_button_rgb, Align2, Frame, Margin, Pos2, Ui, Vec2},EguiContexts};

use crate::util;
```

---

</SwmSnippet>

<SwmSnippet path="/src/ui.rs" line="8">

---

This code snippet initializes the user interface by calling the following functions: `calculate_window_parameters` and `load_user_interface_icons`. It takes in `contexts`, `asset_server`, `ui_state`, and `windows` as arguments.

```renderscript
pub fn init_user_interface(
	mut contexts:	EguiContexts,
	asset_server:	Res<AssetServer>,
	mut ui_state:	ResMut<UIStateManager>,
	windows:		Query<&Window>) {

	calculate_window_parameters(&mut ui_state, &mut contexts, windows.single());
	load_user_interface_icons(&mut ui_state, &asset_server);
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/ui.rs" line="18">

---

This code snippet defines a function called `draw_user_interface` that takes in two mutable arguments: `contexts` of type `EguiContexts` and `ui_state` of type `UIStateManager`. The function displays various UI menus based on the state of `ui_state` such as scene manager menu, play/pause menu, current tool menu (if `show_selected_tool` is true), and visualization menu (if `show_visualization` is true).

```renderscript
pub fn draw_user_interface(
	mut contexts:	EguiContexts,
	mut ui_state:	ResMut<UIStateManager>) {

	// Show "static" UI menus.
	show_scene_manager_menu(&mut ui_state, &mut contexts);
	show_play_pause_menu(&mut ui_state, &mut contexts);

	// Show hideable UI menus.
	if ui_state.show_selected_tool { show_current_tool_menu(&mut ui_state, &mut contexts); }
	if ui_state.show_visualization { show_visualization_menu(&mut ui_state, &mut contexts); }
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/ui.rs" line="31">

---

This code snippet creates a scene manager menu that includes a file manager panel and a tool manager panel. The menu is displayed using the eGUI library and allows users to save/load files and select tools. The `show_scene_manager_menu` function takes in UI state and contexts as parameters and generates the menu interface.

```renderscript
/// Create menu for file saving/loading and tool selection.
fn show_scene_manager_menu(
	ui_state:	&mut UIStateManager,
	contexts:	&mut EguiContexts) {

	/* For each UI icon that we need to load, get their handle from our UI State Manager.  Then,
		convert that into an eGUI-readable egui::Image format!  This is done by iterating through
		the tool icon handles stores in our UI state manager, and then pushing the eGUI-compatible
		texture handle to our list of tool_icons.  These icons will be iterated over later to draw
		each tool button. */
	/* TODO: Maybe move this out of here so we don't do this every frame?  No idea if that is even
		possible. */
	let mut tool_icons: Vec<egui::Image> = Vec::new();
	for i in 0..UI_ICON_COUNT {
		let icon_handle	= ui_state.tool_icon_handles[i].clone_weak();
		tool_icons.push(image_handle_to_egui_texture(
			icon_handle,
			contexts,
			ui_state.icon_size
		));
	}

	// Create an eGUI window.
	egui::Window::new("Scene Manager")
		.frame(ui_state.window_frame)
		.fixed_pos(Pos2 { x: 0.0, y: 0.0 })
		.fixed_size(ui_state.window_size)
		.title_bar(false)
		.resizable(false)
		.show(contexts.ctx_mut(), |ui| {

		// Allow the UI windows to grow to the size of the screen.
		ui.set_width(ui_state.window_size.x);
		ui.set_width(ui_state.window_size.y);

		// Show the file manager panel, a horizontal separator, and the tool manager panel.
		show_file_manager_panel(ui_state, ui);
		ui.separator();
		show_tool_manager_panel(ui_state, ui, &tool_icons);
	});
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/ui.rs" line="73">

---

&nbsp;

```renderscript
/// File management row; align horizontally wrapped.
fn show_file_manager_panel(ui_state: &mut UIStateManager, ui: &mut Ui) {

	ui.horizontal_wrapped(|ui| {

		// "File" scene saving/loading dropdown.
		let file_options		= ["File", "New", "Load", "Save", "Save as"];
		let mut file_selection	= 0;
		egui::ComboBox::from_id_source(0).show_index(
			ui,
			&mut file_selection,
			file_options.len(),
			|i| file_options[i].to_owned()
		);
		// Do stuff when selection changes.
		match file_selection {
			1 => {  },
			2 => {  },
			3 => {  },
			4 => {  },
			_ => {},
		}

		// "Edit" scene dropdown.
		let edit_options		= ["Edit", "Reset", "Clear", "Change Dimensions"];
		let mut edit_selection	= 0;
		egui::ComboBox::from_id_source(1).show_index(
			ui,
			&mut edit_selection,
			edit_options.len(),
			|i| edit_options[i].to_owned()
		);
		// Do stuff when selection changes.
		match edit_selection {
			1 => {  },
			_ => {},
		}

		// "View" scene dropdown.
		let view_options		= ["View", "Current Tool", "Visualization"];
		let mut view_selection	= 0;
		egui::ComboBox::from_id_source(2).show_index(
			ui,
			&mut view_selection,
			view_options.len(),
			|i| view_options[i].to_owned()
		);
		// Do stuff when selection changes.
		match view_selection {
			1 => { ui_state.show_selected_tool = !ui_state.show_selected_tool },
			2 => { ui_state.show_visualization = !ui_state.show_visualization }
			_ => {},
		}
	});
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/ui.rs" line="128">

---

This code snippet defines a function `show_tool_manager_panel` that displays a row of tool buttons horizontally wrapped. The function takes in references to a UIStateManager, a Ui, and a vector of tool icons. It iterates through the tool icons, creating a button for each one with the associated tool icon and text. When a button is clicked, the `selected_tool` field in the UIStateManager is updated with the corresponding tool.

```renderscript

/// Scene/tool management row; align horizontally wrapped.
fn show_tool_manager_panel(
	ui_state:	&mut UIStateManager,
	ui:			&mut Ui,
	tool_icons:	&Vec<egui::Image>) {

	ui.horizontal_wrapped(|ui| {
		// Draw each tool button from our list!
		for i in 0..UI_ICON_COUNT {

			let current_tool: SimTool = i.into();

			// Add a button to the UI and switch the active tool when it is clicked!
			if ui.add(egui::Button::image_and_text(
				tool_icons[i].clone(), current_tool.as_str() )).clicked() {

				ui_state.selected_tool = current_tool;
			}
		}
	});
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/ui.rs" line="151">

---

&nbsp;

```renderscript
/// Show the menu with the current tool's options.
fn show_current_tool_menu(
	ui_state:		&mut UIStateManager,
	contexts:		&mut EguiContexts) {

	// Get the currently selected tool's name.
	let selected_tool_name: String	= ui_state.selected_tool.as_str().to_owned();
	let context_window_name: String	= selected_tool_name + " Options";

	// Create a new eGUI window.
	egui::Window::new(context_window_name)
		.id(egui::Id::from("Tool Selection Window"))
		.frame(ui_state.window_frame)
		.pivot(Align2::CENTER_CENTER)
		.default_pos(Pos2 { x: 0.0, y: ui_state.window_size.y / 2.0 })
		.default_width(0.0)
		.resizable(false)
		.show(contexts.ctx_mut(), |ui| {

		// Align the buttons in this row horizontally from left to right.
		ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {

			// Show different buttons depending on which tool is currently selected.
			match ui_state.selected_tool {
				SimTool::Select			=> {  },
				SimTool::Grab			=> {  },
				SimTool::AddFluid		=> {  },
				SimTool::RemoveFluid	=> {  },
				SimTool::AddWall		=> {  },
				SimTool::RemoveWall		=> {  },
				SimTool::AddFaucet		=> {  },
				SimTool::RemoveFaucet	=> {  },
				SimTool::AddDrain		=> {  },
				SimTool::RemoveDrain	=> {  },
				_						=> {},
			}
		});
	});
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/ui.rs" line="191">

---

This code snippet creates and displays a visualization options menu using the `egui` library. The menu includes checkboxes for showing grid, velocity vectors, and gravity, a dropdown for selecting a fluid color visualization option, color pickers for customizing fluid colors, and sliders for adjusting particle size and gravity direction.

```renderscript
/// Grid/fluid visualization settings menu.
fn show_visualization_menu(ui_state: &mut UIStateManager, contexts: &mut EguiContexts) {

	egui::Window::new("Visualization Options")
		.frame(ui_state.window_frame)
		.pivot(Align2::CENTER_CENTER)
		.default_pos(Pos2 { x: ui_state.window_size.x, y: ui_state.window_size.y / 2.0 })
		.default_width(0.0)
		.resizable(false)
		.show(contexts.ctx_mut(), |ui| {

		// Align the buttons in this row horizontally from left to right.
		ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {

			ui.checkbox(&mut ui_state.show_grid, "Show Grid");
			ui.checkbox(&mut ui_state.show_velocity_vectors, "Show Velocities");
			ui.checkbox(&mut ui_state.show_gravity_vector, "Show Gravity");

			ui.separator();

			// Fluid color visualization option dropdown.
			let color_options = ["Velocity", "Density", "Pressure", "None"];
			egui::ComboBox::from_id_source(0).show_index(
				ui,
				&mut ui_state.fluid_color_variable,
				color_options.len(),
				|i| color_options[i].to_owned()
			);
			ui.horizontal_wrapped(|ui| {
				ui.color_edit_button_rgb(&mut ui_state.fluid_colors[0]);
				ui.color_edit_button_rgb(&mut ui_state.fluid_colors[1]);
				ui.color_edit_button_rgb(&mut ui_state.fluid_colors[2]);
				ui.color_edit_button_rgb(&mut ui_state.fluid_colors[3]);
			});

			ui.separator();

			// Sliders for the particle size and gravity direction.
			ui.add(egui::Slider::new(
				&mut ui_state.particle_physical_size,
				0.1..=10.0
			).text("Particle Size"));

			ui.add(egui::Slider::new(
				&mut ui_state.gravity_direction,
				0.0..=360.0
			).text("Gravity Direction"));
		});
	});
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/ui.rs" line="242">

---

This code snippet defines a function `show_play_pause_menu` that displays a play/pause menu. It creates an egui window with the title 'Play/Pause' and a vertical layout. Inside the window, it displays a button with an image and text that toggles the `is_paused` state when clicked. The image and text for the button depend on the current value of `is_paused`. The function takes `ui_state` and `contexts` as mutable references.

```renderscript
/// Play/pause menu.
fn show_play_pause_menu(
	ui_state:		&mut UIStateManager,
	contexts:		&mut EguiContexts) {

	// Get the icons we need!
	let play_pause_icons: Vec<egui::Image> = Vec::new();
	let play_icon = image_handle_to_egui_texture(
		ui_state.play_pause_icon_handles[0].clone_weak(),
		contexts,
		ui_state.icon_size
	);
	let pause_icon = image_handle_to_egui_texture(
		ui_state.play_pause_icon_handles[1].clone_weak(),
		contexts,
		ui_state.icon_size
	);

	egui::Window::new("Play/Pause")
		.title_bar(false)
		.frame(ui_state.window_frame)
		.fixed_pos(Pos2 { x: ui_state.window_size.x / 2.0, y: ui_state.window_size.y * 0.95 } )
		.pivot(Align2::CENTER_CENTER)
		.default_width(0.0)
		.resizable(false)
		.show(contexts.ctx_mut(), |ui| {

		// Simulation play/pause button.
		ui.vertical_centered(|ui| {

			// Play/pause button!
			let play_pause_icon;
			let play_pause_text;
			if ui_state.is_paused {
				play_pause_icon	= play_icon;
				play_pause_text	= "Paused!";
			} else {
				play_pause_icon	= pause_icon;
				play_pause_text	= "Playing!";
			}
			if ui.add(egui::Button::image_and_text(
				play_pause_icon, play_pause_text)).clicked() {
				ui_state.is_paused = !ui_state.is_paused;
			}
		});
    });
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/ui.rs" line="290">

---

This code snippet defines a constant `UI_ICON_COUNT` with a value of 10. It also defines an enum `SimTool` with several variants including `Select`, `Grab`, `AddFluid`, `RemoveFluid`, `AddWall`, `RemoveWall`, `AddFaucet`, `RemoveFaucet`, `AddDrain`, and `RemoveDrain`. The enum is derived with `Clone`, `Copy`, and `Debug` traits.

```renderscript
const UI_ICON_COUNT: usize = 10;
#[derive(Clone, Copy, Debug)]
pub enum SimTool {
	Select			= 0,
	Grab,
	AddFluid,
	RemoveFluid,
	AddWall,
	RemoveWall,
	AddFaucet,
	RemoveFaucet,
	AddDrain,
	RemoveDrain,
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/ui.rs" line="305">

---

This code defines an implementation of the `Into` trait for the `usize` type. It allows converting a `usize` value into a `SimTool` enum variant. The `into` method matches the `usize` value against different cases and returns the corresponding `SimTool` variant. If the `usize` value doesn't match any of the cases, it prints an error message and defaults to the `SimTool::Select` variant.

```renderscript
impl Into<SimTool> for usize {
	fn into(self) -> SimTool {
		match self {
			0	=> { SimTool::Select },
			1	=> { SimTool::Grab },
			2	=> { SimTool::AddFluid },
			3	=> { SimTool::RemoveFluid },
			4	=> { SimTool::AddWall },
			5	=> { SimTool::RemoveWall },
			6	=> { SimTool::AddFaucet },
			7	=> { SimTool::RemoveFaucet },
			8	=> { SimTool::AddDrain },
			9	=> { SimTool::RemoveDrain },
			_	=> { eprintln!("Invalid SimTool; defaulting to Select!"); SimTool::Select },
		}
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/ui.rs" line="323">

---

This code snippet defines an implementation of the `SimTool` enum and provides a method `as_str` which returns a string representation of each variant of the enum.

```renderscript
impl SimTool {
    fn as_str(&self) -> &'static str {
        match self {
			Self::Select		=> { "Select" },
			Self::Grab			=> { "Grab" },
			Self::AddFluid		=> { "Add Fluid" },
			Self::RemoveFluid	=> { "Remove Fluid" },
			Self::AddWall		=> { "Add Wall" },
			Self::RemoveWall	=> { "Remove Wall" },
			Self::AddFaucet		=> { "Add Faucet" },
			Self::RemoveFaucet	=> { "Remove Faucet" },
			Self::AddDrain		=> { "Add Drain" },
			Self::RemoveDrain	=> { "Remove Drain" },
		}
    }
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/ui.rs" line="340">

---

This code defines a struct called UIStateManager that represents the state of a user interface. It includes various boolean, numeric, and vector fields to control the display and behavior of the UI. The `Default` trait is implemented for the struct, providing a default set of values for its fields.

```renderscript
#[derive(Resource, Debug)]
pub struct UIStateManager {
	show_selected_tool:			bool,
	selected_tool:				SimTool,
	tool_icon_handles:			Vec<Handle<Image>>,

	show_visualization:			bool,
	show_grid:					bool,
	show_velocity_vectors:		bool,
	show_gravity_vector:		bool,
	particle_physical_size:		f32,
	gravity_direction:			f32,
	fluid_color_variable:		usize,
	fluid_colors:				[[f32; 3]; 4],

	is_paused:					bool,
	play_pause_icon_handles:	Vec<Handle<Image>>,

	window_frame:				Frame,
	window_size:				Vec2,
	icon_size:					Vec2,
}

impl Default for UIStateManager {
	fn default() -> UIStateManager {
		UIStateManager {
			show_selected_tool:			true,
			selected_tool:				SimTool::Select,
			tool_icon_handles:			vec![Handle::default(); UI_ICON_COUNT],

			show_visualization:			false,
			show_grid:					false,
			show_velocity_vectors:		false,
			show_gravity_vector:		false,
			particle_physical_size:		1.0,
			gravity_direction:			270.0,
			fluid_color_variable:		0,
			fluid_colors:				[
				[util::JUICE_BLUE.r(), util::JUICE_BLUE.g(), util::JUICE_BLUE.b()],
				[util::JUICE_GREEN.r(), util::JUICE_GREEN.g(), util::JUICE_GREEN.b()],
				[util::JUICE_YELLOW.r(), util::JUICE_YELLOW.g(), util::JUICE_YELLOW.b()],
				[util::JUICE_RED.r(), util::JUICE_RED.g(), util::JUICE_RED.b()],
			],

			is_paused:					false,
			play_pause_icon_handles:	vec![Handle::default(); 2],

			window_frame:				Frame::none(),
			window_size:				Vec2::ZERO,
			icon_size:					Vec2 { x: 30.0, y: 30.0 },
		}
	}
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/ui.rs" line="394">

---

This code snippet calculates the size and frame of the drawing window, and stores the values in the UI state manager. It uses the `Window` class to get the width and height of the window, and then calculates the window size and frame based on the provided padding, border width, and other styling parameters. The calculated values are stored in the `ui_state` struct, which is a mutable reference to the UI state manager. The `contexts` parameter is used to access the Egui styling contexts. The calculated window frame includes information about the fill color, rounding, stroke, inner margin, and outer margin.

```renderscript
/// Determine the size and frame of the drawing window and store it in our UI state manager.
fn calculate_window_parameters(
	ui_state:	&mut UIStateManager,
	contexts:	&mut EguiContexts,
	window:		&Window) {

	// General styling of components for consistency.
	let window_border_width: f32		= 2.5;
	let window_padding: f32				= 10.0;

	// Figure out how large our window is that we are drawing to.
	ui_state.window_size = Vec2 {
		x: window.width() - window_padding - window_border_width,
		y: window.height()
	};
	ui_state.window_frame = Frame {
		fill: contexts.ctx_mut().style().visuals.window_fill(),
        rounding: 10.0.into(),
        stroke: contexts.ctx_mut().style().visuals.widgets.noninteractive.fg_stroke,
		inner_margin: (window_padding / 2.0).into(),
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
	};
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/ui.rs" line="419">

---

This code snippet loads UI icons using Bevy's asset server and stores the loaded image handles into the UI state manager.

```renderscript
/// Using Bevy's asset server, load all UI icons into our UI state manager.
pub fn load_user_interface_icons(
	ui_state:		&mut UIStateManager,
	asset_server:	&AssetServer) {

	// Load all UI icons using Bevy's asset server.
	let icon_handles: [Handle<Image>; UI_ICON_COUNT] = [
		asset_server.load("../assets/ui/icons_og/select_og.png"),
		asset_server.load("../assets/ui/icons_og/grab_og.png"),
		asset_server.load("../assets/ui/icons_og/droplet_og.png"),
		asset_server.load("../assets/ui/icons_og/droplet_og.png"),
		asset_server.load("../assets/ui/icons_og/wall_og.png"),
		asset_server.load("../assets/ui/icons_og/wall_og.png"),
		asset_server.load("../assets/ui/icons_og/faucet_og.png"),
		asset_server.load("../assets/ui/icons_og/faucet_og.png"),
		asset_server.load("../assets/ui/icons_og/swirl_og.png"),
		asset_server.load("../assets/ui/icons_og/swirl_og.png"),
	];
	let play_pause_icon_handles: [Handle<Image>; 2] = [
		asset_server.load("../assets/ui/icons_og/play_og.png"),
		asset_server.load("../assets/ui/icons_og/pause_og.png"),
	];

	// Store all loaded image handles into our UI state manager.
	for i in 0..UI_ICON_COUNT {
		ui_state.tool_icon_handles[i] = icon_handles[i].clone();
	}
	ui_state.play_pause_icon_handles[0] = play_pause_icon_handles[0].clone();
	ui_state.play_pause_icon_handles[1] = play_pause_icon_handles[1].clone();
}
```

---

</SwmSnippet>

<SwmSnippet path="/src/ui.rs" line="450">

---

This code snippet converts a Bevy `Handle<Image>` into an eGUI-compatible `egui::Image`. It adds the image to an eGUI context, converts the eGUI texture ID into an image that eGUI can draw, and returns the resulting image.

```renderscript
/// Convert a Bevy Handle<Image> into an eGUI-compatible eGUI Image!
fn image_handle_to_egui_texture<'a>(
	image_handle:	Handle<Image>,
	contexts:		&mut EguiContexts,
	size:			Vec2) -> bevy_egui::egui::Image<'a> {

	// Add the image to our eGUI context from our UI state manager.
	let select_icon_id = contexts.add_image(image_handle);

	// Convert the eGUI texture ID into an image that eGUI can actually draw.
	let select_icon_img = egui::widgets::Image::new(
		egui::load::SizedTexture::new(
			select_icon_id,
			size
		)
	);

	select_icon_img
}
```

---

</SwmSnippet>

&nbsp;

<SwmMeta version="3.0.0" repo-id="Z2l0aHViJTNBJTNBSnVpY2VCb3glM0ElM0FBZXJvbmF1dGljYWwtQXhvbG90bHM=" repo-name="JuiceBox"><sup>Powered by [Swimm](https://app.swimm.io/)</sup></SwmMeta>
