use bevy::{
	prelude::*,
	core_pipeline::prelude::ClearColor,
	ecs::query::QueryItem,
	sprite::MaterialMesh2dBundle,
	render::{
		RenderApp, 
		extract_component::ExtractComponent
	},
};
use crate::util;

pub struct JuiceRenderer;
impl Plugin for JuiceRenderer {

	fn build(&self, app: &mut App) {
		app.insert_resource(ClearColor(util::JUICE_BLUE));

		app.add_systems(Startup, setup_renderer);

		let mut render_app = app.sub_app_mut(RenderApp);
		// render_app.add_systems(Render, something_probably_goes_here_but_idk_what_yet);
	}
}

/// Custom rendering pipeline initialization.
fn setup_renderer(
	mut commands:	Commands,
	mut meshes:		ResMut<Assets<Mesh>>,
	mut materials:	ResMut<Assets<ColorMaterial>>) {

	// Spawn a camera to view our simulation world!
	commands.spawn(Camera2dBundle::default());

	// Spawn test particle!
	let particle_color_material: ColorMaterial = ColorMaterial::from(
		util::generate_color_from_gradient(
			util::JUICE_GREEN, 
			util::JUICE_YELLOW, 
			util::JUICE_RED, 
			0.5
		)
	);
	commands.spawn(MaterialMesh2dBundle {
		mesh:		meshes.add(shape::Circle::new(10.0).into()).into(),
		material:	materials.add(particle_color_material),
		transform:	Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
		..default()
	});
}

// Vector with all InstanceData which we will force feed to the GPU!
#[derive(Component, Clone, ExtractComponent)]	// BUG: Might need to re-implement ExtractComponent.
struct InstanceMaterialData(Vec<InstanceData>);

#[derive(Clone)]
// Enable C/C++ struct packing to prevent byte offsets from messing the whole darn thing up.
#[repr(C)]
struct InstanceData {
    position:	Vec2,
    velocity:	Vec2,
}