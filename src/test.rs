use bevy::prelude::*;

pub struct HelloWorld;
impl Plugin for HelloWorld
{
	fn build(&self, app: &mut App)
	{
		app.insert_resource(GreetingTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
		app.add_systems(Update, hello_world);
	}
}

#[derive(Resource)]
struct GreetingTimer(Timer);

fn hello_world(time: Res<Time>, mut timer: ResMut<GreetingTimer>)
{
	if timer.0.tick(time.delta()).just_finished()
	{
		println!("Hello, world!");
	}
}

// comment