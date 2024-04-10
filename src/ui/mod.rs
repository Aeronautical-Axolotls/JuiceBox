mod interface;
mod interaction;

use std::mem::transmute;

use bevy::{asset::{AssetServer, Assets, Handle}, ecs::system::{Query, Res, ResMut, Resource}, prelude::default, render::{color::Color, texture::Image}, ui::FlexWrap, window::Window};
use bevy_egui::{egui::{self, color_picker::color_edit_button_rgb, Align2, Frame, Margin, Pos2, Ui, Vec2},EguiContexts};
use bevy::prelude::*;
use crate::{events::{PlayPauseStepEvent}, util};
use self::interaction::{change_cursor_icon, handle_input, handle_camera_input};
use crate::events::{ResetEvent, UseToolEvent};

pub struct JuiceUI;
impl Plugin for JuiceUI {

	fn build(&self, app: &mut App) {
        app.insert_resource(UIStateManager::default());
        app.add_systems(Startup, init_ui);

		app.add_systems(Update, update_ui);
        app.add_systems(Update, handle_input);
		app.add_systems(Update, handle_camera_input);
		app.add_systems(Update, change_cursor_icon);

		app.add_event::<ResetEvent>();
        app.add_event::<UseToolEvent>();
		app.add_event::<PlayPauseStepEvent>();
	}
}

const UI_ICON_COUNT: usize = 13;
#[derive(Clone, Copy, Debug)]
pub enum SimTool {
	Select			= 0,
	Camera,
	Zoom,
	Gravity,
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
			1	=> { SimTool::Camera },
			2	=> { SimTool::Zoom },
			3	=> { SimTool::Gravity },
			4	=> { SimTool::Grab },
			5	=> { SimTool::AddFluid },
			6	=> { SimTool::RemoveFluid },
			7	=> { SimTool::AddWall },
			8	=> { SimTool::RemoveWall },
			9	=> { SimTool::AddFaucet },
			10	=> { SimTool::RemoveFaucet },
			11	=> { SimTool::AddDrain },
			12	=> { SimTool::RemoveDrain },
			_	=> { eprintln!("Invalid SimTool; defaulting to Select!"); SimTool::Select },
		}
	}
}

impl SimTool {
    fn as_str(&self) -> &'static str {
        match self {
			Self::Select		=> { "Select" },
			Self::Camera		=> { "Camera" },
			Self::Zoom			=> { "Zoom" },
			Self::Gravity		=> { "Gravity" },
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
	pub	show_selected_tool:			bool,
	pub	selected_tool:				SimTool,
	pub	tool_icon_handles:			Vec<Handle<Image>>,
	pub	zoom_slider:				f32,
	pub	grab_slider_radius:			f32,
	pub	add_remove_fluid_radius:	f32,
	pub	add_fluid_density:			f32,
	pub	faucet_direction:			f32,
	pub	faucet_radius:				f32,
	pub	faucet_pressure:			f32,

	pub	show_visualization:			bool,
	pub	show_grid:					bool,
	pub	show_velocity_vectors:		bool,
	pub	show_gravity_vector:		bool,
	pub	particle_physical_size:		f32,
	pub	gravity_direction:			f32,
	pub	gravity_magnitude:			f32,
	pub fluid_color_variable:		usize,
	pub	fluid_colors:				[[f32; 3]; 4],

	pub	is_paused:					bool,
	pub	play_pause_icon_handles:	Vec<Handle<Image>>,

	pub	window_frame:				Frame,
	pub	window_size:				Vec2,
	pub	icon_size:					Vec2,

	pub	show_informational:			bool,
}

impl Default for UIStateManager {
	fn default() -> UIStateManager {
		UIStateManager {
			// Currently selected tool menu.
			show_selected_tool:			true,
			selected_tool:				SimTool::Select,
			tool_icon_handles:			vec![Handle::default(); UI_ICON_COUNT],
			zoom_slider:				1.0,
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
			gravity_magnitude:			9.81,
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

			// Show the informational window at the start of the program?
			show_informational:			true,
		}
	}
}

pub fn init_ui(
	mut contexts:	EguiContexts,
	asset_server:	Res<AssetServer>,
	mut ui_state:	ResMut<UIStateManager>) {

    interface::init_user_interface(contexts, asset_server, ui_state);

}

pub fn update_ui(
	mut contexts:	EguiContexts,
	mut ui_state:	ResMut<UIStateManager>,
	windows:		Query<&Window>) {

    interface::draw_user_interface(contexts, ui_state, windows);

}
