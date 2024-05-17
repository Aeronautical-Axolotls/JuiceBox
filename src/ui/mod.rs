mod interface;
mod interaction;

use std::mem::transmute;

use bevy::{asset::{AssetServer, Assets, Handle}, ecs::system::{Query, Res, ResMut, Resource}, prelude::default, render::{color::Color, texture::Image}, ui::FlexWrap, window::Window};
use bevy_egui::{egui::{self, color_picker::color_edit_button_rgb, Align2, Frame, Margin, Pos2, Ui, Vec2},EguiContexts};
use bevy::prelude::*;

use crate::file_system::{self, JuiceStates};
use crate::{events::{ModifyVisualizationEvent, PlayPauseStepEvent}, util};
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
		app.add_event::<ModifyVisualizationEvent>();
	}
}

const UI_ICON_COUNT: usize = 12;
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SimTool {
	Camera = 0,
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
			0	=> { SimTool::Camera },
			1	=> { SimTool::Zoom },
			2	=> { SimTool::Gravity },
			3	=> { SimTool::Grab },
			4	=> { SimTool::AddFluid },
			5	=> { SimTool::RemoveFluid },
			6	=> { SimTool::AddWall },
			7	=> { SimTool::RemoveWall },
			8	=> { SimTool::AddFaucet },
			9	=> { SimTool::RemoveFaucet },
			10	=> { SimTool::AddDrain },
			11	=> { SimTool::RemoveDrain },
			_	=> { eprintln!("Invalid SimTool; defaulting to Grab!"); SimTool::Grab },
		}
	}
}

impl SimTool {
    fn as_str(&self) -> &'static str {
        match self {
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
    pub drain_radius:               f32,
    pub drain_pressure:             f32,

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

	pub file_state:					JuiceStates,
	pub reset:						bool,
	pub new:						bool,
	pub load:						bool,
	pub save:						bool,
	pub save_as:					bool,
}

impl Default for UIStateManager {
	fn default() -> UIStateManager {
		UIStateManager {
			// Currently selected tool menu.
			show_selected_tool:			true,
			selected_tool:				SimTool::AddFluid,
			tool_icon_handles:			vec![Handle::default(); UI_ICON_COUNT],
			zoom_slider:				1.0,
			grab_slider_radius:			15.0,
			add_remove_fluid_radius:	25.0,
			add_fluid_density:			0.5,
			faucet_direction:			320.0,
			faucet_radius:				1.0,
			faucet_pressure:			35.0,
            drain_radius:               10.5,
            drain_pressure:             30.0,

			// Visualization menu.
			show_visualization:			true,
			show_grid:					false,
			show_velocity_vectors:		false,
			show_gravity_vector:		false,
			particle_physical_size:		0.4,
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

			// File and scene stuff.
			file_state:					JuiceStates::Running,
			reset:						false,
			new:						false,
			load:						false,
			save:						false,
			save_as:					false,
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
	windows:		Query<&Window>,
	ev_viz:			EventWriter<ModifyVisualizationEvent>,
	ev_pause:		EventWriter<PlayPauseStepEvent>,
	mut current_file: ResMut<file_system::CurrentFile>,
	mut file_state: ResMut<NextState<file_system::JuiceStates>>) {

    interface::draw_user_interface(contexts, ui_state, windows, ev_viz, ev_pause, current_file.as_mut(), file_state);

}
