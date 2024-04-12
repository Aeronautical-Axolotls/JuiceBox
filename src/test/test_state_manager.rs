use core::panic;

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use crate::juice_renderer::{self, draw_selection_circle};
use crate::simulation::sim_state_manager::{
	select_particles,
	delete_particle,
    add_faucet,
    add_drain,
};
use crate::simulation::{step_simulation_once, SimSurfaceDirection};
use crate::simulation::{
    self,
	SimConstraints,
	SimParticle,
	SimGrid,
    SimFaucet,
    SimGridCellType,
    SimDrain,
	sim_state_manager::{
		add_particles_in_radius,
		add_particle,
	},
};
use crate::util::{cartesian_to_polar, get_cursor_position, polar_to_cartesian};

/// Create a simulation layout for testing.
pub fn construct_test_simulation_layout(
	constraints:	&mut SimConstraints,
	grid:			&mut SimGrid,
	mut commands:	&mut Commands) {

	// Create a lil cup
	// grid.set_grid_cell_type(40, 32, SimGridCellType::Solid);
	// grid.set_grid_cell_type(41, 32, SimGridCellType::Solid);
	// grid.set_grid_cell_type(42, 32, SimGridCellType::Solid);
	// grid.set_grid_cell_type(43, 32, SimGridCellType::Solid);
	// grid.set_grid_cell_type(43, 33, SimGridCellType::Solid);
	// grid.set_grid_cell_type(43, 34, SimGridCellType::Solid);
	// grid.set_grid_cell_type(43, 35, SimGridCellType::Solid);
	// grid.set_grid_cell_type(43, 36, SimGridCellType::Solid);
	// grid.set_grid_cell_type(43, 37, SimGridCellType::Solid);
	// grid.set_grid_cell_type(43, 38, SimGridCellType::Solid);
	// grid.set_grid_cell_type(43, 39, SimGridCellType::Solid);
	// grid.set_grid_cell_type(42, 39, SimGridCellType::Solid);
	// grid.set_grid_cell_type(41, 39, SimGridCellType::Solid);
	// grid.set_grid_cell_type(40, 39, SimGridCellType::Solid);

	// // Create a BIG wall
	// grid.set_grid_cell_type(40, 14, SimGridCellType::Solid);
	// grid.set_grid_cell_type(41, 14, SimGridCellType::Solid);
	// grid.set_grid_cell_type(42, 14, SimGridCellType::Solid);
	// grid.set_grid_cell_type(43, 14, SimGridCellType::Solid);
	// grid.set_grid_cell_type(44, 14, SimGridCellType::Solid);
	// grid.set_grid_cell_type(45, 14, SimGridCellType::Solid);
	// grid.set_grid_cell_type(46, 14, SimGridCellType::Solid);
	// grid.set_grid_cell_type(47, 14, SimGridCellType::Solid);
	// grid.set_grid_cell_type(48, 14, SimGridCellType::Solid);

	// // Generate walls around simulation bounds.
    // for i in 0..50 {
    //     grid.set_grid_cell_type(49, i, SimGridCellType::Solid);
    //     grid.set_grid_cell_type(0, i, SimGridCellType::Solid);
    //     grid.set_grid_cell_type(i, 0, SimGridCellType::Solid);
    //     grid.set_grid_cell_type(i, 49, SimGridCellType::Solid);
    // }

    // Add faucet
    // let faucet_pos = Vec2::new(grid.cell_size as f32, grid.cell_size as f32 * 20.0);
    // let surface_direction = None;

    // add_faucet(&mut commands, grid, faucet_pos, surface_direction).ok();

    // Add Drain
    // let drain_pos = Vec2::new(grid.cell_size as f32 * 25.0, 0.0);
    // let surface_direction = None;

    // add_drain(commands, grid, drain_pos, surface_direction, grid.cell_size as f32).ok();

	// Spawn a small test group of particles at the center of the screen.
	let grid_center: Vec2 = Vec2 {
		x: (grid.dimensions.1 * grid.cell_size) as f32 * 0.5,
		y: (grid.dimensions.0 * grid.cell_size) as f32 * 0.5,
	};

	// let _ = add_particle(
	// 	&mut commands,
	// 	constraints,
	// 	grid,
	// 	Vec2 { x: grid_center[0] * 1.5, y: grid_center[1] * 0.75 },
	// 	Vec2 { x: 0.0, y: 0.0 }
	// );

	let _moar_test_particles = add_particles_in_radius(
		commands,
        constraints,
		grid,
		1.75,
		100.0,
		Vec2 { x: grid_center[0] * 0.7, y: grid_center[1] * 0.85 },
		Vec2::ZERO
	);

	println!("Creating a test simulation with {} particles...", constraints.particle_count);
}

/// Debugging state controller.
pub fn debug_state_controller(
	mut commands:		Commands,
	keys:				Res<Input<KeyCode>>,
	mouse:				Res<Input<MouseButton>>,
	mut mouse_motion:	EventReader<MouseMotion>,
	windows:			Query<&Window>,
	cameras:			Query<(&Camera, &GlobalTransform)>,
	mut constraints:	ResMut<SimConstraints>,
	mut grid:			ResMut<SimGrid>,
	mut particles:		Query<(Entity, &mut SimParticle)>) {

	// Reset simulation when we press R.
	if keys.just_pressed(KeyCode::R) {
		crate::simulation::reset_simulation_to_default(
			&mut commands,
			constraints.as_mut(),
			grid.as_mut(),
			&mut particles
		);
		construct_test_simulation_layout(
			constraints.as_mut(),
			grid.as_mut(),
			&mut commands
		);
		return;
	}

	// Rotate/scale gravity when we press the arrow keys.
	let gravity_rotation: i8	= keys.pressed(KeyCode::Right) as i8 -
									keys.pressed(KeyCode::Left) as i8;
	let gravity_magnitude: i8	= keys.pressed(KeyCode::Up) as i8 -
									keys.pressed(KeyCode::Down) as i8;
	let mut polar_gravity: Vec2	= cartesian_to_polar(constraints.gravity);
	polar_gravity.x				+= 200.0 * gravity_magnitude as f32 * constraints.timestep;
	polar_gravity.y				+= 4.0 * gravity_rotation as f32 * constraints.timestep;

	// Limit the magnitude of the vector to prevent ugly behavior near 0.0.
	polar_gravity.x				= f32::max(0.0, polar_gravity.x);
	constraints.gravity			= polar_to_cartesian(polar_gravity);

	// Place/remove grid cells if the mouse is clicked on a cell.
	let should_place_cell: bool		= mouse.pressed(MouseButton::Left);
	let should_remove_cell: bool	= mouse.pressed(MouseButton::Right);

	// Get the mouse's motion between this and the last frame.
	let mut cursor_delta: Vec2 = Vec2::ZERO;
	for event in mouse_motion.read() {
		cursor_delta.x = event.delta.x;
		cursor_delta.y = event.delta.y;
	}

	if should_place_cell {
		let cursor_position: Vec2	= get_cursor_position(&windows, &cameras);
		let cell_coordinates: Vec2	= grid.get_cell_coordinates_from_position(&cursor_position);
		let _ = grid.set_grid_cell_type(
			cell_coordinates.x as usize,
			cell_coordinates.y as usize,
			SimGridCellType::Solid
		);

		// Delete all particles in the cell we are turning into a solid.
		let lookup_index: usize = grid.get_lookup_index(cell_coordinates);
		grid.delete_all_particles_in_cell(
			&mut commands,
			constraints.as_mut(),
			&particles,
			lookup_index
		);

	} else if should_remove_cell {
		let cursor_position: Vec2	= get_cursor_position(&windows, &cameras);
		let cell_coordinates: Vec2	= grid.get_cell_coordinates_from_position(&cursor_position);
		let _ = grid.set_grid_cell_type(
			cell_coordinates.x as usize,
			cell_coordinates.y as usize,
			SimGridCellType::Air
		);
	}
}

/// Simulation state manager initialization.
pub fn test_setup(
	mut commands:		Commands,
	mut constraints:	ResMut<SimConstraints>,
	mut grid:			ResMut<SimGrid>) {

	construct_test_simulation_layout(
		constraints.as_mut(),
		grid.as_mut(),
		&mut commands
	);

}

pub fn test_update(
	mut constraints:	ResMut<SimConstraints>,
	mut grid:			ResMut<SimGrid>,
	mut particles:		Query<(Entity, &mut SimParticle)>,
    faucets:			Query<(Entity, &mut SimFaucet)>,
    drains:		        Query<(Entity, &SimDrain)>,
	mut commands:       Commands,
    ) {

	// let delta_time: f32 = time.delta().as_millis() as f32 * 0.001;
	let fixed_timestep: f32 = constraints.timestep;

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

/// Test particle selection.
pub fn test_select_particles(
	commands:			&mut Commands,
	constraints:		&mut SimConstraints,
	mut grid:			&mut SimGrid,
	mut particles:		&Query<(Entity, &mut SimParticle)>,
	windows:			&Query<&Window>,
	cameras:			&Query<(&Camera, &GlobalTransform)>,
	mut gizmos:			&mut Gizmos) {

	let radius: f32				= 55.0;
	let cursor_position: Vec2	= get_cursor_position(windows, cameras);
	draw_selection_circle(gizmos, cursor_position, radius, Color::YELLOW);

	// Test particle selection.
	let selected_particles: Vec<Entity> = select_particles(
		particles,
		grid,
		cursor_position,
		radius
	);

	// Delete them to prove it worked!
	for particle in selected_particles.iter() {
		let _ = delete_particle(commands, constraints, particles, grid, *particle);
	}
}

/// Test grid cell selection.
pub fn test_select_grid_cells(
	commands:			&mut Commands,
	constraints:		&mut SimConstraints,
	mut grid:			&mut SimGrid,
	mut particles:		&Query<(Entity, &mut SimParticle)>,
	windows:			&Query<&Window>,
	cameras:			&Query<(&Camera, &GlobalTransform)>,
	mut gizmos:			&mut Gizmos) {

	let radius: f32				= 55.0;
	let cursor_position: Vec2	= get_cursor_position(windows, cameras);

	// Test cell selection.
	let selected_cells: Vec<Vec2> = grid.select_grid_cells(cursor_position, radius);
	for i in 0..selected_cells.len() {

		let half_cell_size: f32 = grid.cell_size as f32 * 0.5;
		let mut cell_position: Vec2 = grid.get_cell_position_from_coordinates(selected_cells[i]);

		cell_position.x += half_cell_size;
		cell_position.y += half_cell_size;

		gizmos.rect_2d(
			cell_position,
			0.0,
			Vec2 {
				x: grid.cell_size as f32,
				y: grid.cell_size as f32,
			},
			Color::ANTIQUE_WHITE
		);
	}
}

/// runs the add_faucet() function for testing
fn test_add_faucet_update(
	mut commands:	Commands,
	asset_server:	Res<AssetServer>,
	mut grid:		ResMut<SimGrid>) {

    let faucet_pos = Vec2::new(grid.cell_size as f32, grid.cell_size as f32 * 10.0);
    let surface_direction = None;

    let Err(e) = simulation::sim_state_manager::add_faucet(&mut commands, &asset_server, grid.as_mut(), faucet_pos, surface_direction, 1.0, Vec2::ZERO) else {

        return;
    };

    panic!("{}", e);

}

/// runs the add_faucet() function for testing
fn test_add_drain_update(
	mut commands:		Commands,
	asset_server:		&AssetServer,
	mut grid:			ResMut<SimGrid>
    ) {

    let drain_pos = Vec2::new(grid.cell_size as f32 * 25.0, 1.0);
    let surface_direction = Some(SimSurfaceDirection::South);
    let drain_radius = grid.cell_size as f32;

    let Err(e) = simulation::sim_state_manager::add_drain(&mut commands, &asset_server, grid.as_mut(), drain_pos, surface_direction, drain_radius) else {

        return;
    };

    panic!("{}", e);

}

#[test]
fn add_faucet_test() {

    //First we setup the test world in bevy
    let mut juicebox_test = App::new();

    // Add our constraints and grid
    juicebox_test.insert_resource(SimGrid::default());
    juicebox_test.insert_resource(SimConstraints::default());

    // Add our test setup environment
	juicebox_test.add_systems(Startup, test_setup);
	juicebox_test.add_systems(Update, test_update);

    // Add the test function for our add_faucet state change
    juicebox_test.add_systems(Update, test_add_faucet_update);

    // Then we run 1 step through the simulation with update()
    juicebox_test.update();

    // Verify we have added a faucet
    let faucet = juicebox_test.world.component_id::<SimFaucet>();

    assert_ne!(None, faucet);

}

#[test]
fn run_faucet_test() {

    //First we setup the test world in bevy
    let mut juicebox_test = App::new();

    juicebox_test.insert_resource(SimGrid::default());
    juicebox_test.insert_resource(SimConstraints::default());

	juicebox_test.add_systems(Startup, test_setup);
	juicebox_test.add_systems(Update, test_update);

    // Add the test function for our add_faucet state change
    juicebox_test.add_systems(Update, test_add_faucet_update);

    // Then we run 1 step through the simulation with update()
    juicebox_test.update();

    // Get particle count before faucet has ran
    let before_count = juicebox_test.world.resource::<SimConstraints>().particle_count;

    juicebox_test.update();

    // Get particle count after faucet has ran
    let after_count = juicebox_test.world.resource::<SimConstraints>().particle_count;

    // Verify that the amount of particles has changed,
    // thus, the faucet successfully ran
    assert_ne!(after_count, before_count);
}

#[test]
fn add_drain_test() {

    //First we setup the test world in bevy
    let mut juicebox_test = App::new();

    // Add our constraints and grid
    juicebox_test.insert_resource(SimGrid::default());
    juicebox_test.insert_resource(SimConstraints::default());

    // Add our test setup environment
	juicebox_test.add_systems(Startup, test_setup);
	juicebox_test.add_systems(Update, test_update);

    // Add the test function for our add_drain state change
    juicebox_test.add_systems(Update, test_add_drain_update);

    // Then we run 1 step through the simulation with update()
    juicebox_test.update();

    // Verify we have added a drain
    let drain = juicebox_test.world.component_id::<SimDrain>();

    assert_ne!(None, drain);

}

#[test]
fn drain_drain_test() {

    //First we setup the test world in bevy
    let mut juicebox_test = App::new();

    juicebox_test.insert_resource(SimGrid::default());
    juicebox_test.insert_resource(SimConstraints::default());

	juicebox_test.add_systems(Startup, test_setup);
	juicebox_test.add_systems(Update, test_update);

    // Add the test function for our add_drain state change
    juicebox_test.add_systems(Update, test_add_drain_update);

    // Then we run 1 step through the simulation with update()
    juicebox_test.update();

    // Get particle count before drain has drained
    let before_count = juicebox_test.world.resource::<SimConstraints>().particle_count;

    // Run a couple times to let the fluid fall
    juicebox_test.update();
    juicebox_test.update();
    juicebox_test.update();
    juicebox_test.update();
    juicebox_test.update();
    juicebox_test.update();

    // Get particle count after drain has drained
    let after_count = juicebox_test.world.resource::<SimConstraints>().particle_count;

    // Verify that the amount of particles has changed,
    // thus, the drain successfully drained
    assert_ne!(after_count, before_count);
}
