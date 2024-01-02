use bevy::{
	prelude::*,
	core_pipeline::prelude::ClearColor, render::color,
};
use crate::{
	util::{self, vector_magnitude},
	simulation::sim_state_manager::{
		SimParticle,
		SimGrid,
	},
};

pub struct JuiceRenderer;
impl Plugin for JuiceRenderer {

	fn build(&self, app: &mut App) {
		app.insert_resource(ClearColor(util::JUICE_SKY_BLUE));
		app.insert_resource(FluidRenderData::default());
		app.insert_resource(GridRenderData::default());

		app.add_systems(Startup, setup_renderer);
		
		app.add_systems(Update, update_particle_position);
		app.add_systems(Update, update_particle_color);
		app.add_systems(Update, update_particle_size);
		
		app.add_systems(Update, draw_grid_vectors);
		app.add_systems(Update, draw_grid_cells);
	}
}

enum FluidColorRenderType	{ Arbitrary, Velocity, Pressure }
enum FluidGridVectorType	{ Velocity }

#[derive(Resource)]
struct FluidRenderData {
	color_render_type:	FluidColorRenderType,
	arbitrary_color:	Color,
	velocity_magnitude_color_scale:	f32,
	pressure_magnitude_color_scale:	f32,
}

impl Default for FluidRenderData {
	
	fn default() -> Self {
		Self {
			color_render_type:	FluidColorRenderType::Velocity,
			arbitrary_color:	util::JUICE_YELLOW,
			velocity_magnitude_color_scale:	10.0,
			pressure_magnitude_color_scale:	10.0,
		}
	}
}

#[derive(Resource)]
struct GridRenderData {
	draw_grid:			bool,
	grid_color:			Color,
	
	draw_grid_vectors:	bool,
	grid_vector_type:	FluidGridVectorType,
	grid_vector_color:	Color,
}

impl Default for GridRenderData {
	
	fn default() -> Self {
		Self {
			draw_grid:			true,
			grid_color:			Color::WHITE,
			
			draw_grid_vectors:	true,
			grid_vector_type:	FluidGridVectorType::Velocity,
			grid_vector_color:	Color::BLACK,
		}
	}
}

/// Custom rendering pipeline initialization.
fn setup_renderer(mut commands: Commands, grid: Res<SimGrid>) {
	
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

/** Creates and links a new sprite to the specified particle; **Must be called each time a new 
	particle is added to the simulation!** */
pub fn link_particle_sprite(mut commands: &mut Commands, particle: Entity) {
	commands.entity(particle).insert(SpriteBundle::default());
}

/// Update the visual transform of all particles to be rendered.
fn update_particle_position(mut particles: Query<(&SimParticle, &mut Transform)>) {
	
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

/// Update the size of all particles to be rendered.
fn update_particle_size(mut particles: Query<(&SimParticle, &mut Sprite)>) {
	
	for (_, mut sprite) in particles.iter_mut() {
		let size: f32 = 10.0;
		sprite.custom_size = Some(Vec2::splat(size));
	}
}

/// Update the color of all particles to be rendered.
fn update_particle_color(
	mut particles: Query<(&SimParticle, &mut Sprite)>,
	grid: Res<SimGrid>,
	particle_render_data: Res<FluidRenderData>) {
	
	match particle_render_data.color_render_type {
		FluidColorRenderType::Velocity	=> color_particles_by_velocity(
			particles,
			particle_render_data.velocity_magnitude_color_scale
		),
		FluidColorRenderType::Pressure	=> color_particles_by_pressure(
			particles,
			grid.as_ref(),
			particle_render_data.pressure_magnitude_color_scale
		),
		FluidColorRenderType::Arbitrary	=> color_particles(
			particles, 
			particle_render_data.arbitrary_color
		),
	}
}

/// Color all particles in the simulation by their velocities.
fn color_particles_by_velocity(
	mut particles: Query<(&SimParticle, &mut Sprite)>,
	velocity_magnitude_color_scale: f32) {

	for (particle, mut sprite) in particles.iter_mut() {
		
		let color: Color = util::generate_color_from_gradient(
			vec![util::JUICE_BLUE, util::JUICE_GREEN, util::JUICE_YELLOW, util::JUICE_RED],
			util::vector_magnitude(particle.velocity) / velocity_magnitude_color_scale,
		);
		
		sprite.color = color;
	}
}

/// Color all particles in the simulation by their pressures.
fn color_particles_by_pressure(
	mut particles: Query<(&SimParticle, &mut Sprite)>,
	grid: &SimGrid,
	pressure_magnitude_color_scale: f32) {
	
	for (particle, mut sprite) in particles.iter_mut() {
		
		let cell_pos: Vec2	= grid.get_cell_coordinates_from_position(&particle.position);
		let cell_row: usize	= cell_pos[1] as usize;
		let cell_col: usize	= cell_pos[0] as usize;
		
		let color: Color = util::generate_color_from_gradient(
			vec![util::JUICE_BLUE, util::JUICE_GREEN, util::JUICE_YELLOW, util::JUICE_RED],
			grid.cell_center[cell_row][cell_col] / pressure_magnitude_color_scale,
		);
		sprite.color = color;
	}
}

/// Color all particles in the simulation as anything you want!
fn color_particles(mut particles: Query<(&SimParticle, &mut Sprite)>, color: Color) {
	
	for (_, mut sprite) in particles.iter_mut() {
		sprite.color = color;
	}
}

/// Draw grid cells based on SimGrid using Bevy's Gizmos!
fn draw_grid_cells(grid: Res<SimGrid>, grid_render_data: Res<GridRenderData>, mut gizmos: Gizmos) {
	
	if !grid_render_data.draw_grid {
		return;
	}
	
	let grid_width: f32		= (grid.dimensions.0 * grid.cell_size) as f32;
	let grid_height: f32	= (grid.dimensions.1 * grid.cell_size) as f32;
	
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

/// Draw velocity vectors based on SimGrid using Bevy's Gizmos!
fn draw_grid_vectors(
	grid:				Res<SimGrid>,
	grid_render_data:	Res<GridRenderData>,
	mut gizmos:			Gizmos) {
	
	if !grid_render_data.draw_grid_vectors {
		return;
	}
	
	for x in 0..grid.dimensions.0 {
		for y in 0..grid.dimensions.1 {
			
			// Find the center of each grid cell to draw the vector arrows.
			let half_cell_size: f32 = (grid.cell_size as f32) / 2.0;
			let cell_center_position: Vec2 = Vec2 {
				x: (x as f32) * (grid.cell_size as f32) + half_cell_size,
				y: (y as f32) * (grid.cell_size as f32) + half_cell_size,
			};
			
			/* Indices for each column/row of each u/v velocity component on the grid.  Note that 
				because each cell has two velocity components going in either direction, the 
				vectors containing said components are one element larger in either rows or 
				columns.  This fact prevents the following code from going out of bounds, so long 
				as grid.velocity_u and grid.velocity_v are constructed properly. */
			let column_u0: usize	= x as usize;
			let column_u1: usize	= (x + 1) as usize;
			let row_u: usize		= y as usize;
			
			let row_v0: usize		= y as usize;
			let row_v1: usize		= (y + 1) as usize;
			let column_v: usize		= x as usize;
			
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
			
			draw_vector_arrow(
				cell_center_position, 
				velocity_vector_polar[1],
				velocity_vector_polar[0],
				grid_render_data.grid_vector_color,
				&mut gizmos);
		}
	}
}

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