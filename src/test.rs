use bevy::prelude::*;
use bevy::sprite::{Mesh2dHandle, MaterialMesh2dBundle};

pub struct HelloWorld;
impl Plugin for HelloWorld
{
	fn build(&self, app: &mut App)
	{
		app.insert_resource(GreetingTimer(Timer::from_seconds(2.0, TimerMode::Repeating)));
		app.add_systems(Update, hello_world);
		app.add_systems(Startup, test_setup);
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

fn test_setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>, asset_server: Res<AssetServer>)
{
	// Set up triangle mesh.
	let texture_handle				= asset_server.load("test.png");
	let quad_mesh: Mesh				= create_quad();
	let mesh_handle: Mesh2dHandle	= meshes.add(quad_mesh).into();
	
	// Spawn camera and triangle.
	commands.spawn(Camera2dBundle::default());
	commands.spawn(MaterialMesh2dBundle
	{
		mesh:		mesh_handle.clone(), 
		transform:	Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)).with_scale(Vec3::splat(512.0)), 
		material:	materials.add(ColorMaterial::from(texture_handle)), 
		..default()
	});
}

fn create_quad() -> Mesh
{
	// Create default quad mesh with vertex positions & vertex indices.
	let mut mesh = Mesh::from(shape::Quad::default());
	
	// Insert vertex color attributes for each of the 4 vertices of the mesh.
	mesh.insert_attribute
	(
		Mesh::ATTRIBUTE_COLOR, 
		vec!
		[
			Color::RED.as_rgba_f32(),
        	Color::GREEN.as_rgba_f32(),
        	Color::BLUE.as_rgba_f32(),
        	Color::WHITE.as_rgba_f32(),
		], 
	);
	
	println!("Quad created!");
	
	mesh
}