use bevy::prelude::*;

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