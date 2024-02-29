use std::mem::transmute;

use bevy::{asset::{AssetServer, Assets, Handle}, ecs::system::{Query, Res, ResMut, Resource}, prelude::default, render::texture::Image, ui::FlexWrap, window::Window};
use bevy_egui::{egui::{self, color_picker::color_edit_button_rgb, Align2, Frame, Margin, Pos2, Vec2},EguiContexts};

pub fn draw_user_interface(
	mut contexts:	EguiContexts,
	mut ui_state:	ResMut<UIStateManager>,
	windows:		Query<&Window>,
	images:			ResMut<Assets<Image>>) {

	// General styling of components for consistency.
	let window_border_width: f32		= 2.5;
	let window_padding: f32				= 10.0;
	let icon_size: [f32; 2]				= [30.0; 2];
	let default_button_padding: Vec2	= Vec2 { x: 4.0, y: 1.0 };
	let combo_dropdown_padding: Vec2	= Vec2 { x: 10.0, y: 9.0 };

	// Window stuff.
	let window				= windows.single();
	let window_size: Vec2	= Vec2 {
		x: window.width() - window_padding - window_border_width,
		y: window.height()
	};
	let window_frame: Frame = Frame {
		fill: contexts.ctx_mut().style().visuals.window_fill(),
        rounding: 10.0.into(),
        stroke: contexts.ctx_mut().style().visuals.widgets.noninteractive.fg_stroke,
		inner_margin: (window_padding / 2.0).into(),
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
	};

	/* For each UI icon that we need to load, get their handle from our UI State Manager.  Then,
		convert that into an eGUI-readable egui::Image format!  This is done by iterating through
		the tool icon handles stores in our UI state manager, and then pushing the eGUI-compatible
		texture handle to our list of tool_icons.  These icons will be iterated over later to draw
		each tool button. */
	let mut tool_icons: Vec<egui::Image>		= Vec::new();
	let mut play_pause_icons: Vec<egui::Image>	= Vec::new();

	let tool_names: [&str; UI_ICON_COUNT] = [
		"Select",
		"Grab",
		"Add Fluid",
		"Remove Fluid",
		"Add Walls",
		"Remove Walls",
		"Add Faucet",
		"Remove Faucet",
		"Add Drain",
		"Remove Drain"
	];
	for i in 0..UI_ICON_COUNT {
		let icon_handle	= ui_state.tool_icon_handles[i].clone_weak();
		tool_icons.push(image_handle_to_egui_texture(icon_handle, &mut contexts, icon_size));
	}

	// Now do the same thing for the play/pause icons.
	let play_icon = image_handle_to_egui_texture(
		ui_state.play_pause_icon_handles[0].clone_weak(),
		&mut contexts,
		icon_size
	);
	let pause_icon = image_handle_to_egui_texture(
		ui_state.play_pause_icon_handles[1].clone_weak(),
		&mut contexts,
		icon_size
	);

	// Create top window for file saving/loading and tool selecting.
	egui::Window::new("Scene Manager")
		.frame(window_frame)
		.fixed_pos(Pos2 { x: 0.0, y: 0.0 })
		.fixed_size(window_size)
		.title_bar(false)
		.resizable(false)
		.show(contexts.ctx_mut(), |ui| {

		// Allow the UI windows to grow to the size of the screen.
		ui.set_width(window_size.x);
		ui.set_width(window_size.y);

		// File management row; align horizontally wrapped.
		ui.horizontal_wrapped(|ui| {

			// "File" scene saving/loading dropdown.
			let file_options		= ["File", "New", "Load", "Save", "Save as"];
			let mut file_selection	= 0;
			egui::ComboBox::from_id_source(0).show_index(
				ui,
				&mut file_selection,
				file_options.len(),
				|i| file_options[i].to_owned()
			);
			match file_selection {
				1 => {  },
				2 => {  },
				3 => {  },
				4 => {  },
				_ => {},
			}

			// "Edit" scene dropdown.
			let edit_options		= ["Edit", "Reset"];
			let mut edit_selection	= 0;
			egui::ComboBox::from_id_source(1).show_index(
				ui,
				&mut edit_selection,
				edit_options.len(),
				|i| edit_options[i].to_owned()
			);
			match edit_selection {
				1 => {  },
				_ => {},
			}

			// "View" scene dropdown.
			let view_options		= ["View", "Current Tool", "Visualization"];
			let mut view_selection	= 0;
			egui::ComboBox::from_id_source(2).show_index(
				ui,
				&mut view_selection,
				view_options.len(),
				|i| view_options[i].to_owned()
			);
			match view_selection {
				1 => { ui_state.show_selected_tool = !ui_state.show_selected_tool },
				2 => { ui_state.show_visualization = !ui_state.show_visualization }
				_ => {},
			}
		});

		// Draws a horizontal line between the file and tool management sections of this menu.
		ui.separator();

		// Scene/tool management row; align horizontally wrapped.
		ui.horizontal_wrapped(|ui| {
			// Draw each tool button from our list!
			for i in 0..UI_ICON_COUNT {
				// Add a button to the UI and check it for click events!
				if ui.add(egui::Button::image_and_text(
					tool_icons[i].clone(), tool_names[i])).clicked() {

					/* Convert i into a SimTool so we can check which tool we are using!  If i is
						not a valid SimTool, then choose the "Select" tool. */
					if i < UI_ICON_COUNT {
						ui_state.selected_tool = unsafe { transmute(i as u8) };
						println!("Selected Tool: {:?}", ui_state.selected_tool);
					} else {
						ui_state.selected_tool = SimTool::Select;
						eprintln!("Invalid tool selected!  Defaulting to \"Select\".");
					}
				}
			}
		});
	});

	// Draw the selected tool's context window.
	if ui_state.show_selected_tool {

		// Get the currently selected tool's name.
		let selected_tool_name: String	= tool_names[ui_state.selected_tool as usize].to_owned();
		let context_window_name: String	= selected_tool_name + " Options";

		egui::Window::new(context_window_name)
			.id(egui::Id::from("Tool Selection Window"))
			.frame(window_frame)
			.default_pos(Pos2 { x: 0.0, y: window_size.y / 2.0 })
			.default_width(0.0)
			.show(contexts.ctx_mut(), |ui| {

			// Align the buttons in this row horizontally from left to right.
			ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {

				// Color picker!
				ui.color_edit_button_rgb(&mut ui_state.color_picker_rgb);

				// Tool size!
				let mut tool_size: f32 = 10.0;
				ui.add(egui::Slider::new(&mut tool_size, 1.0..=500.0));
			});
		});
	}

	// Fluid/grid visualization settings window.
	if ui_state.show_visualization {

		egui::Window::new("Visualization Options")
			.frame(window_frame)
			.default_pos(Pos2 { x: window_size.x, y: window_size.y / 2.0 })
			.default_width(0.0)
			.show(contexts.ctx_mut(), |ui| {

			// Align the buttons in this row horizontally from left to right.
			ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {

				// Color picker!
				ui.color_edit_button_rgb(&mut ui_state.color_picker_rgb);

				// Tool size!
				let mut tool_size: f32 = 10.0;
				ui.add(egui::Slider::new(&mut tool_size, 1.0..=500.0));
			});
		});
	}

	// Play/pause button menu.
	egui::Window::new("Play/Pause")
		.title_bar(false)
		.frame(window_frame)
		.fixed_pos(Pos2 { x: window_size.x / 2.0, y: window_size.y * 0.95 } )
		.pivot(Align2::CENTER_CENTER)
		.default_width(0.0)
		.resizable(false)
		.show(contexts.ctx_mut(), |ui| {

		// Simulation play/pause button.
		ui.vertical_centered(|ui| {

			// Play/pause button!
			let play_pause_icon;
			let play_pause_text;
			if ui_state.is_paused {
				play_pause_icon	= play_icon;
				play_pause_text	= "Paused!";
			} else {
				play_pause_icon	= pause_icon;
				play_pause_text	= "Playing!";
			}
			if ui.add(egui::Button::image_and_text(
				play_pause_icon, play_pause_text)).clicked() {
				ui_state.is_paused = !ui_state.is_paused;
			}
		});
    });
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

impl Into<usize> for SimTool {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Resource, Debug)]
pub struct UIStateManager {
	show_selected_tool:			bool,
	selected_tool:				SimTool,
	tool_icon_handles:			Vec<Handle<Image>>,
	color_picker_rgb:			[f32; 3],

	show_visualization:			bool,
	is_paused:					bool,
	play_pause_icon_handles:	Vec<Handle<Image>>,
}

impl Default for UIStateManager {
	fn default() -> UIStateManager {
		UIStateManager {
			show_selected_tool:			true,
			selected_tool:				SimTool::Select,
			tool_icon_handles:			vec![Handle::default(); UI_ICON_COUNT],
			color_picker_rgb:			[1.0, 0.0, 1.0],

			show_visualization:			false,
			is_paused:					false,
			play_pause_icon_handles:	vec![Handle::default(); 2],
		}
	}
}

pub fn load_user_interface_icons(
	mut ui_state:	ResMut<UIStateManager>,
	asset_server:	Res<AssetServer>) {

	// Load all UI icons using Bevy's asset server.
	let icon_handles: [Handle<Image>; UI_ICON_COUNT] = [
		asset_server.load("../assets/ui/icons_og/select_og.png"),
		asset_server.load("../assets/ui/icons_og/grab_og.png"),
		asset_server.load("../assets/ui/icons_og/droplet_og.png"),
		asset_server.load("../assets/ui/icons_og/droplet_og.png"),
		asset_server.load("../assets/ui/icons_og/wall_og.png"),
		asset_server.load("../assets/ui/icons_og/wall_og.png"),
		asset_server.load("../assets/ui/icons_og/faucet_og.png"),
		asset_server.load("../assets/ui/icons_og/faucet_og.png"),
		asset_server.load("../assets/ui/icons_og/swirl_og.png"),
		asset_server.load("../assets/ui/icons_og/swirl_og.png"),
	];
	let play_pause_icon_handles: [Handle<Image>; 2] = [
		asset_server.load("../assets/ui/icons_og/play_og.png"),
		asset_server.load("../assets/ui/icons_og/pause_og.png"),
	];

	// Store all loaded image handles into our UI state manager.
	for i in 0..UI_ICON_COUNT {
		ui_state.tool_icon_handles[i] = icon_handles[i].clone();
	}
	ui_state.play_pause_icon_handles[0] = play_pause_icon_handles[0].clone();
	ui_state.play_pause_icon_handles[1] = play_pause_icon_handles[1].clone();
}

/// Convert a Bevy Handle<Image> into an eGUI-compatible eGUI Image!
fn image_handle_to_egui_texture<'a>(
	image_handle:	Handle<Image>,
	contexts:		&mut EguiContexts,
	size:			[f32; 2]) -> bevy_egui::egui::Image<'a> {

	// Add the image to our eGUI context from our UI state manager.
	let select_icon_id = contexts.add_image(image_handle);

	// Convert the eGUI texture ID into an image that eGUI can actually draw.
	let select_icon_img = egui::widgets::Image::new(
		egui::load::SizedTexture::new(
			select_icon_id,
			size
		)
	);

	select_icon_img
}