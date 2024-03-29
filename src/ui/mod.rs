mod interaction;

use std::mem::transmute;

use bevy::{asset::{AssetServer, Assets, Handle}, ecs::system::{Query, Res, ResMut, Resource}, prelude::default, render::{color::Color, texture::Image}, ui::FlexWrap, window::Window};
use bevy_egui::{egui::{self, color_picker::color_edit_button_rgb, Align2, Frame, Margin, Pos2, Ui, Vec2},EguiContexts};

use crate::util;

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

	// Show "static" UI menus.
	show_scene_manager_menu(&mut ui_state, &mut contexts);
	show_play_pause_menu(&mut ui_state, &mut contexts);

	// Show hideable UI menus.
	if ui_state.show_selected_tool { show_current_tool_menu(&mut ui_state, &mut contexts); }
	if ui_state.show_visualization { show_visualization_menu(&mut ui_state, &mut contexts); }
}

/// Create menu for file saving/loading and tool selection.
fn show_scene_manager_menu(
	ui_state:	&mut UIStateManager,
	contexts:	&mut EguiContexts) {

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
			contexts,
			ui_state.icon_size
		));
	}

	// Create an eGUI window.
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

		// Show the file manager panel, a horizontal separator, and the tool manager panel.
		show_file_manager_panel(ui_state, ui);
		ui.separator();
		show_tool_manager_panel(ui_state, ui, &tool_icons);
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
		// Do stuff when selection changes.
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
		// Do stuff when selection changes.
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
		// Do stuff when selection changes.
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

			// Add a button to the UI and switch the active tool when it is clicked!
			if ui.add(egui::Button::image_and_text(
				tool_icons[i].clone(), current_tool.as_str() )).clicked() {

				ui_state.selected_tool = current_tool;
			}
		}
	});
}

/// Show the menu with the current tool's options.
fn show_current_tool_menu(
	ui_state:		&mut UIStateManager,
	contexts:		&mut EguiContexts) {

	// Get the currently selected tool's name.
	let selected_tool_name: String	= ui_state.selected_tool.as_str().to_owned();
	let context_window_name: String	= selected_tool_name + " Options";

	// Create a new eGUI window.
	egui::Window::new(context_window_name)
		.id(egui::Id::from("Tool Selection Window"))
		.frame(ui_state.window_frame)
		.pivot(Align2::CENTER_CENTER)
		.default_pos(Pos2 { x: 0.0, y: ui_state.window_size.y / 2.0 })
		.default_width(0.0)
		.resizable(false)
		.show(contexts.ctx_mut(), |ui| {

		// Align the buttons in this row horizontally from left to right.
		ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {

			// Show different buttons depending on which tool is currently selected.
			match ui_state.selected_tool {

				// For the Select tool, show some text as there are no options for Select.
				SimTool::Select			=> {
					ui.label("No options available for the Select tool!");
				},

				// For the Grab tool, show a slider for the grabbing radius.
				SimTool::Grab			=> {
					ui.add(egui::Slider::new(
						&mut ui_state.grab_slider_radius,
						1.0..=500.0
					).text("Grab Radius"));
				},

				// For the Add Fluid tool, show density and radius sliders.
				SimTool::AddFluid		=> {
					ui.add(egui::Slider::new(
						&mut ui_state.add_remove_fluid_radius,
						1.0..=500.0
					).text("Brush Radius"));
					ui.add(egui::Slider::new(
						&mut ui_state.add_fluid_density,
						0.01..=10.0
					).text("Fluid Density"));
				},

				// For the Remove Fluid tool, show a radius slider.
				SimTool::RemoveFluid	=> {
					ui.add(egui::Slider::new(
						&mut ui_state.add_remove_fluid_radius,
						1.0..=500.0
					).text("Eraser Radius"));
				},

				// For the Add Wall tool, show some text as there are no options for Add Wall.
				SimTool::AddWall		=> {
					ui.label("No options available for the Add Wall tool!");
				},

				// For the Remove Wall tool, show some text as there are no options for Remove Wall.
				SimTool::RemoveWall		=> {
					ui.label("No options available for the Remove Wall tool!");
				},

				/* For the Add Faucet tool, show sliders for the direction, volume, and speed
					of the fluid coming out of the faucet. */
				SimTool::AddFaucet		=> {
					ui.add(egui::Slider::new(
						&mut ui_state.faucet_direction,
						0.0..=360.0
					).text("Faucet Direction"));
					ui.add(egui::Slider::new(
						&mut ui_state.faucet_radius,
						1.0..=10.0
					).text("Faucet Pipe Diameter"));
					ui.add(egui::Slider::new(
						&mut ui_state.faucet_pressure,
						0.0..=100.0
					).text("Faucet Pressure"));
				},

				// For the Remove Faucet tool, show some text as there are no options for Remove Faucet.
				SimTool::RemoveFaucet	=> {
					ui.label("No options available for the Remove Faucet tool!");
				},

				/* For the Add Drain tool, show a sucking radius radius slider and a pressure slider
					for controlling how intensely a drain pulls fluid inwards. */
				SimTool::AddDrain		=> {
					ui.add(egui::Slider::new(
						&mut ui_state.faucet_pressure,
						0.0..=100.0
					).text("Drain Suck Radius"));
					ui.add(egui::Slider::new(
						&mut ui_state.faucet_pressure,
						0.0..=100.0
					).text("Drain Pressure"));
				},

				// For the Remove Drain tool, show some text as there are no options for Remove Drain.
				SimTool::RemoveDrain	=> {
					ui.label("No options available for the Remove Drain tool!");
				},

				// It should literally not be possible for this final case to happen.
				_						=> {
					ui.label("If you are seeing this message, something is wrong :(");
				},
			}
		});
	});
}

/// Grid/fluid visualization settings menu.
fn show_visualization_menu(ui_state: &mut UIStateManager, contexts: &mut EguiContexts) {

	egui::Window::new("Visualization Options")
		.frame(ui_state.window_frame)
		.pivot(Align2::CENTER_CENTER)
		.default_pos(Pos2 { x: ui_state.window_size.x, y: ui_state.window_size.y / 2.0 })
		.default_width(0.0)
		.resizable(false)
		.show(contexts.ctx_mut(), |ui| {

		// Align the buttons in this row horizontally from left to right.
		ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {

			ui.checkbox(&mut ui_state.show_grid, "Show Grid");
			ui.checkbox(&mut ui_state.show_velocity_vectors, "Show Velocities");
			ui.checkbox(&mut ui_state.show_gravity_vector, "Show Gravity");

			ui.separator();

			// Fluid color visualization option dropdown.
			let color_options = ["Velocity", "Density", "Pressure", "None"];
			egui::ComboBox::from_id_source(0).show_index(
				ui,
				&mut ui_state.fluid_color_variable,
				color_options.len(),
				|i| color_options[i].to_owned()
			);
			ui.horizontal_wrapped(|ui| {
				ui.color_edit_button_rgb(&mut ui_state.fluid_colors[0]);
				ui.color_edit_button_rgb(&mut ui_state.fluid_colors[1]);
				ui.color_edit_button_rgb(&mut ui_state.fluid_colors[2]);
				ui.color_edit_button_rgb(&mut ui_state.fluid_colors[3]);
			});

			ui.separator();

			// Sliders for the particle size and gravity direction.
			ui.add(egui::Slider::new(
				&mut ui_state.particle_physical_size,
				0.1..=10.0
			).text("Particle Size"));

			ui.add(egui::Slider::new(
				&mut ui_state.gravity_direction,
				0.0..=360.0
			).text("Gravity Direction"));
		});
	});
}

/// Play/pause menu.
fn show_play_pause_menu(
	ui_state:		&mut UIStateManager,
	contexts:		&mut EguiContexts) {

	// Get the icons we need!
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
