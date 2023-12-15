use bevy::{
	prelude::*,
	core_pipeline::prelude::ClearColor,
};
use crate::{
	util,
	simulation::sim_state_manager::{
		SimParticle,
		SimGrid,
	}
};

pub struct JuiceRenderer;
impl Plugin for JuiceRenderer {

	fn build(&self, app: &mut App) {
		app.insert_resource(ClearColor(util::JUICE_SKY_BLUE));

		app.add_systems(Startup, setup_renderer);
		
		app.add_systems(Update, update_particle_position);
		app.add_systems(Update, update_particle_color);
		app.add_systems(Update, update_particle_size);
		
		app.add_systems(Update, draw_grid_vectors);
	}
}

/// Custom rendering pipeline initialization.
fn setup_renderer(mut commands: Commands) {

	// Spawn a camera to view our simulation world!
	commands.spawn(Camera2dBundle::default());
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

/// Update the color of all particles to be rendered.
fn update_particle_color(mut particles: Query<(&SimParticle, &mut Sprite)>) {
	
	for (particle, mut sprite) in particles.iter_mut() {
		let color: Color = util::generate_color_from_gradient(
			vec![util::JUICE_BLUE, util::JUICE_GREEN, util::JUICE_YELLOW, util::JUICE_RED],
			util::vector_magnitude(particle.velocity)
		);
		
		sprite.color = color;
	}
}

/// Update the size of all particles to be rendered.
fn update_particle_size(mut particles: Query<(&SimParticle, &mut Sprite)>) {
	
	for (_, mut sprite) in particles.iter_mut() {
		let size: f32 = 10.0;
		sprite.custom_size = Some(Vec2::splat(size));
	}
}

/// Draw velocity vectors based on SimGrid using Bevy's Gizmos!
fn draw_grid_vectors(grid: Res<SimGrid>, mut gizmos: Gizmos) {
	
}

/// Helper function to draw a vector arrow using Bevy's Gizmos.
fn draw_vector_arrow(tail_position: Vec2, direction_degrees: f32, magnitude: f32, gizmos: &mut Gizmos) {
	
	// Construct main ray of arrow.
	let direction_rads: f32	= util::degrees_to_radians(direction_degrees);
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
	gizmos.line_2d(tail_position, head_position, Color::BLACK);
	gizmos.line_2d(head_position, arrow_left_position, Color::BLACK);
	gizmos.line_2d(head_position, arrow_right_position, Color::BLACK);
}