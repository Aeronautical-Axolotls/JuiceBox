use std::mem::transmute;

use bevy::{asset::{AssetServer, Assets, Handle}, ecs::system::{Query, Res, ResMut, Resource}, prelude::default, render::texture::Image, ui::FlexWrap, window::Window};
use bevy_egui::{egui::{self, color_picker::color_edit_button_rgb, Frame, Pos2, Vec2},EguiContexts};

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
	selected_tool:			SimTool,
	color_picker_rgb:		[f32; 3],
	tool_icon_handles:		Vec<Handle<Image>>,
}

impl Default for UIStateManager {
	fn default() -> UIStateManager {
		UIStateManager {
			selected_tool:			SimTool::Select,
			color_picker_rgb:		[1.0, 0.0, 1.0],
			tool_icon_handles:		vec![Handle::default(); UI_ICON_COUNT],
		}
	}
}

pub fn load_user_interface_icons(
	mut ui_state:	ResMut<UIStateManager>,
	asset_server:	Res<AssetServer>) {

	// Load all images into the program using the asset server.
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

	// Store all loaded image handles into our UI state manager.
	for i in 0..UI_ICON_COUNT {
		ui_state.tool_icon_handles[i] = icon_handles[i].clone();
	}
}

pub fn draw_user_interface(
	mut contexts:	EguiContexts,
	mut ui_state:	ResMut<UIStateManager>,
	windows:		Query<&Window>,
	images:			ResMut<Assets<Image>>) {

	// egui_extras::install_image_loaders(contexts);

	let window				= windows.single();
	let border_width: f32	= 2.5;
	let window_padding: f32	= 10.0;
	let window_size: Vec2	= Vec2 {
		x: window.width() - window_padding - border_width,
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
		convert that into an eGUI-readable egui::Image format! */
	let icon_size: [f32; 2] = [30.0; 2];

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
	let mut tool_icons: Vec<egui::Image> = Vec::new();
	for i in 0..UI_ICON_COUNT {
		let icon_handle	= ui_state.tool_icon_handles[i].clone_weak();
		tool_icons.push(image_handle_to_egui_texture(icon_handle, &mut contexts, icon_size));
	}


	// Create top window for file saving/loading and tool selecting.
	egui::Window::new("Scene Manager")
		.frame(window_frame)
		.fixed_pos(Pos2 { x: 0.0, y: 0.0 })
		.fixed_size(window_size)
		.resizable(false)
		.show(contexts.ctx_mut(), |ui| {

		// Allow the UI windows to grow to the size of the screen.
		ui.set_width(window_size.x);
		ui.set_width(window_size.y);

		// Align the buttons in this row horizontally from left to right.
		ui.horizontal_wrapped(|ui| {

			// "File" scene saving/loading dropdown.
			let alternatives = ["File", "Save", "Save as", "Load", "Reset"];
			let mut selected = 0;
			egui::ComboBox::from_label("").show_index(
				ui,
				&mut selected,
				alternatives.len(),
				|i| alternatives[i].to_owned()
			);

			// Loop through each tool button.
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

	// Context window for the currently selected tool's options.
	let selected_tool_name: String	= tool_names[ui_state.selected_tool as usize].to_owned();
	let context_window_name: String	= selected_tool_name + " Options";

	egui::Window::new(context_window_name)
		.id(egui::Id::from("Tool Selection Window"))
		.frame(window_frame)
		.default_pos(Pos2 { x: 0.0, y: window_size.y / 2.0 })
		.show(contexts.ctx_mut(), |ui| {

		// Align the buttons in this row horizontally from left to right.
		ui.with_layout(egui::Layout::top_down_justified(egui::Align::TOP), |ui| {

			// Color picker!
			ui.color_edit_button_rgb(&mut ui_state.color_picker_rgb);

			// Tool size!
			let mut tool_size: f32 = 10.0;
			ui.add(egui::Slider::new(&mut tool_size, 1.0..=500.0));
		});
    });

	// Scrub bar for time travel!
	egui::Window::new("Time Travel")
		.title_bar(false)
		.frame(window_frame)
		.fixed_pos(Pos2 { x: border_width, y: window_size.y } )
		.fixed_size(window_size)
		.show(contexts.ctx_mut(), |ui| {

		// Allow the UI window to grow to the size of the screen.
		ui.set_width(window_size.x);
		ui.set_width(window_size.y);

		// Make this UI bar take up as little space as possible.
		ui.with_layout(egui::Layout::left_to_right(egui::Align::Min), |ui| {

			// Simulation play/pause and scrub bar.
			ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
				if ui.button("Play/Pause").clicked() {

				}
				let mut scrubbar: f32 = 0.0;
				ui.add(egui::Slider::new(&mut scrubbar, -120000.0..=0.0).text("Go back in time!")
					.show_value(false));
			});

			// JuiceBox branding!
			ui.with_layout(egui::Layout::right_to_left(egui::Align::TOP), |ui| {
				ui.add(egui::Label::new("JuiceBox: Spilling Encouraged!"));
			});
		});
    });
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