mod interface;
mod interaction;

use std::mem::transmute;

use bevy::{asset::{AssetServer, Assets, Handle}, ecs::system::{Query, Res, ResMut, Resource}, prelude::default, render::{color::Color, texture::Image}, ui::FlexWrap, window::Window};
use bevy_egui::{egui::{self, color_picker::color_edit_button_rgb, Align2, Frame, Margin, Pos2, Ui, Vec2},EguiContexts};
use bevy::prelude::*;
use crate::util;

pub struct JuiceUI;
impl Plugin for JuiceUI {

	fn build(&self, app: &mut App) {
        app.insert_resource(UIStateManager::default());
        app.add_systems(Startup, init_ui);
        app.add_systems(Update, update_ui);
	}
}

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

#[derive(Resource, Debug)]
pub struct UIStateManager {
	show_selected_tool:			bool,
	selected_tool:				SimTool,
	tool_icon_handles:			Vec<Handle<Image>>,
	grab_slider_radius:			f32,
	add_remove_fluid_radius:	f32,
	add_fluid_density:			f32,
	faucet_direction:			f32,
	faucet_radius:				f32,
	faucet_pressure:			f32,

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
			// Currently selected tool menu.
			show_selected_tool:			true,
			selected_tool:				SimTool::Select,
			tool_icon_handles:			vec![Handle::default(); UI_ICON_COUNT],
			grab_slider_radius:			10.0,
			add_remove_fluid_radius:	25.0,
			add_fluid_density:			1.75,
			faucet_direction:			45.0,
			faucet_radius:				2.0,
			faucet_pressure:			10.0,

			// Visualization menu.
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

			// Play/pause.
			is_paused:					false,
			play_pause_icon_handles:	vec![Handle::default(); 2],

			// Used for coherency between EGUI menus.
			window_frame:				Frame::none(),
			window_size:				Vec2::ZERO,
			icon_size:					Vec2 { x: 30.0, y: 30.0 },
		}
	}
}

pub fn init_ui(
	mut contexts:	EguiContexts,
	asset_server:	Res<AssetServer>,
	mut ui_state:	ResMut<UIStateManager>,
	windows:		Query<&Window>) {

    interface::init_user_interface(contexts, asset_server, ui_state, windows);

}

pub fn update_ui(
	mut contexts:	EguiContexts,
	mut ui_state:	ResMut<UIStateManager>) {

    interface::draw_user_interface(contexts, ui_state);

}
