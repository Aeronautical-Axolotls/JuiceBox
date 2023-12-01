use bevy::prelude::*;
use bevy::math::Vec2;
use std::f32::consts::PI;

pub struct SimPhysicsEngine;
impl Plugin for SimPhysicsEngine {
	fn build(&self, app: &mut App) {
		app.add_systems(Startup, setup);
		app.add_systems(Update, update);
	}
}

fn setup(mut _commands: Commands) {
	
}

fn update(mut _commands: Commands) {
	
}

/** Converts a polar vector with direction and magnitude into a cartesian vector with x and y 
	components; returns said cartesian vector.  This function intended for use with the gravity 
	system, hence the seemingly odd choice of parameters. */
fn _polar_to_cartesian(direction_degrees: u16, magnitude: f32) -> Vec2
{
	let direction_rads: f32 = (direction_degrees as f32) * (PI / 180.0);
	
	let result: Vec2 = Vec2 {
		x: direction_rads.cos() * magnitude, 
		y: direction_rads.sin() * magnitude, 
	};
	
	result
}