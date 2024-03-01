use std::mem::transmute;

use bevy::{asset::{AssetServer, Assets, Handle}, ecs::system::{Query, Res, ResMut, Resource}, prelude::default, render::texture::Image, ui::FlexWrap, window::Window};
use bevy_egui::{egui::{self, color_picker::color_edit_button_rgb, Align2, Frame, Margin, Pos2, Ui, Vec2},EguiContexts};

pub fn init_user_interface(
	mut contexts:	EguiContexts,
	asset_server:	Res<AssetServer>,
	mut ui_state:	ResMut<UIStateManager>,
	windows:		Query<&Window>) {

	calculate_window_parameters(&mut ui_state, &mut contexts, windows.single());
	load_user_interface_icons(&mut ui_state, &asset_server);
}

pub fn draw_user_interface(
	mut contexts:	EguiContexts,
	mut ui_state:	ResMut<UIStateManager>) {

	/* For each UI icon that we need to load, get their handle from our UI State Manager.  Then,
		convert that into an eGUI-readable egui::Image format!  This is done by iterating through
		the tool icon handles stores in our UI state manager, and then pushing the eGUI-compatible
		texture handle to our list of tool_icons.  These icons will be iterated over later to draw
		each tool button. */
	/* TODO: Maybe move this out of here so we don't do this every frame?  No idea if that is even
		possible. */
	let mut tool_icons: Vec<egui::Image> = Vec::new();
	for i in 0..UI_ICON_COUNT {
		let icon_handle	= ui_state.tool_icon_handles[i].clone_weak();
		tool_icons.push(image_handle_to_egui_texture(
			icon_handle,
			&mut contexts,
			ui_state.icon_size
		));
	}

	show_scene_manager_menu(&mut ui_state, &mut contexts, &tool_icons);
	if ui_state.show_selected_tool {
		show_current_tool_menu(&mut ui_state, &mut contexts);
	}
	if ui_state.show_visualization {
		show_visualization_menu(&ui_state, &mut contexts);
	}
	show_play_pause_menu(&mut ui_state, &mut contexts);
}

/// Create menu for file saving/loading and tool selection.
fn show_scene_manager_menu(
	ui_state:	&mut UIStateManager,
	contexts:	&mut EguiContexts,
	tool_icons:	&Vec<egui::Image>) {

	egui::Window::new("Scene Manager")
		.frame(ui_state.window_frame)
		.fixed_pos(Pos2 { x: 0.0, y: 0.0 })
		.fixed_size(ui_state.window_size)
		.title_bar(false)
		.resizable(false)
		.show(contexts.ctx_mut(), |ui| {

		// Allow the UI windows to grow to the size of the screen.
		ui.set_width(ui_state.window_size.x);
		ui.set_width(ui_state.window_size.y);

		show_file_manager_panel(ui_state, ui);
		ui.separator();
		show_tool_manager_panel(ui_state, ui, tool_icons);
	});
}

/// File management row; align horizontally wrapped.
fn show_file_manager_panel(ui_state: &mut UIStateManager, ui: &mut Ui) {

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
		let edit_options		= ["Edit", "Reset", "Clear", "Change Dimensions"];
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
}

/// Scene/tool management row; align horizontally wrapped.
fn show_tool_manager_panel(
	ui_state:	&mut UIStateManager,
	ui:			&mut Ui,
	tool_icons:	&Vec<egui::Image>) {

	ui.horizontal_wrapped(|ui| {
		// Draw each tool button from our list!
		for i in 0..UI_ICON_COUNT {

			let current_tool: SimTool = i.into();

			// Add a button to the UI and check it for click events!
			if ui.add(egui::Button::image_and_text(
				tool_icons[i].clone(), current_tool.as_str() )).clicked() {

				ui_state.selected_tool = current_tool;
			}
		}
	});
}

fn show_current_tool_menu(
	ui_state:		&mut UIStateManager,
	contexts:		&mut EguiContexts) {

	// Get the currently selected tool's name.
	let selected_tool_name: String	= ui_state.selected_tool.as_str().to_owned();
	let context_window_name: String	= selected_tool_name + " Options";

	egui::Window::new(context_window_name)
		.id(egui::Id::from("Tool Selection Window"))
		.frame(ui_state.window_frame)
		.pivot(Align2::CENTER_CENTER)
		.default_pos(Pos2 { x: 0.0, y: ui_state.window_size.y / 2.0 })
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

/// Grid/fluid visualization settings menu.
fn show_visualization_menu(ui_state: &UIStateManager, contexts: &mut EguiContexts) {

	egui::Window::new("Visualization Options")
		.frame(ui_state.window_frame)
		.pivot(Align2::CENTER_CENTER)
		.default_pos(Pos2 { x: ui_state.window_size.x, y: ui_state.window_size.y / 2.0 })
		.default_width(0.0)
		.show(contexts.ctx_mut(), |ui| {

		// Align the buttons in this row horizontally from left to right.
		ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {

			// Fluid color visualization option dropdown.
			let color_options = ["Velocity", "Density", "Pressure", "None"];
		});
	});
}

fn show_play_pause_menu(
	ui_state:		&mut UIStateManager,
	contexts:		&mut EguiContexts) {

	let play_pause_icons: Vec<egui::Image> = Vec::new();
	let play_icon = image_handle_to_egui_texture(
		ui_state.play_pause_icon_handles[0].clone_weak(),
		contexts,
		ui_state.icon_size
	);
	let pause_icon = image_handle_to_egui_texture(
		ui_state.play_pause_icon_handles[1].clone_weak(),
		contexts,
		ui_state.icon_size
	);

	egui::Window::new("Play/Pause")
		.title_bar(false)
		.frame(ui_state.window_frame)
		.fixed_pos(Pos2 { x: ui_state.window_size.x / 2.0, y: ui_state.window_size.y * 0.95 } )
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
	color_picker_rgb:			[f32; 3],

	show_visualization:			bool,
	is_paused:					bool,
	play_pause_icon_handles:	Vec<Handle<Image>>,

	window_frame:				Frame,
	window_size:				Vec2,
	icon_size:					Vec2,
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

			window_frame:				Frame::none(),
			window_size:				Vec2::ZERO,
			icon_size:					Vec2 { x: 30.0, y: 30.0 },
		}
	}
}

/// Determine the size and frame of the drawing window and store it in our UI state manager.
fn calculate_window_parameters(
	ui_state:	&mut UIStateManager,
	contexts:	&mut EguiContexts,
	window:		&Window) {

	// General styling of components for consistency.
	let window_border_width: f32		= 2.5;
	let window_padding: f32				= 10.0;

	// Figure out how large our window is that we are drawing to.
	ui_state.window_size = Vec2 {
		x: window.width() - window_padding - window_border_width,
		y: window.height()
	};
	ui_state.window_frame = Frame {
		fill: contexts.ctx_mut().style().visuals.window_fill(),
        rounding: 10.0.into(),
        stroke: contexts.ctx_mut().style().visuals.widgets.noninteractive.fg_stroke,
		inner_margin: (window_padding / 2.0).into(),
        outer_margin: 0.5.into(), // so the stroke is within the bounds
        ..Default::default()
	};
}

/// Using Bevy's asset server, load all UI icons into our UI state manager.
pub fn load_user_interface_icons(
	ui_state:		&mut UIStateManager,
	asset_server:	&AssetServer) {

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
	size:			Vec2) -> bevy_egui::egui::Image<'a> {

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