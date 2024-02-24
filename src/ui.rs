use bevy::{ecs::system::Query, ui::FlexWrap, window::Window};
use bevy_egui::{egui::{self, color_picker::color_edit_button_rgb, Frame, Pos2, Vec2},EguiContexts};

pub fn ui_base(mut contexts: EguiContexts, windows: Query<&Window>) {

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

	egui::Window::new("Scene Manager")
		.frame(window_frame)
		.fixed_pos(Pos2 { x: 0.0, y: 0.0 })
		.show(contexts.ctx_mut(), |ui| {

		// Allow the UI windows to grow to the size of the screen.
		ui.set_width(window_size.x);
		ui.set_width(window_size.y);

		// Align the buttons in this row horizontally from left to right.
		ui.horizontal_wrapped(|ui| {

			// Scene saving/loading dropdown.
			let alternatives = ["File", "Save", "Save as", "Load", "Reset"];
			let mut selected = 0;
			egui::ComboBox::from_label("").show_index(
				ui,
				&mut selected,
				alternatives.len(),
				|i| alternatives[i].to_owned()
			);

			// Tools buttons.
			if ui.button("Move Camera").clicked() {

			}
			if ui.button("Grab Fluid").clicked() {

			}
			if ui.button("Add/Remove Fluid").clicked() {

			}
			if ui.button("Add/Remove Walls").clicked() {

			}
			if ui.button("Add/Remove Faucets").clicked() {

			}
			if ui.button("Add/Remove Drains").clicked() {

			}
		});
	});

	// Context window for currently selected tool's options.
	let tool_name: String			= "[Tool Name]".to_owned();
	let context_window_name: String	= tool_name + " Options";
	egui::Window::new(context_window_name)
		.frame(window_frame)
		.show(contexts.ctx_mut(), |ui| {

		// Align the buttons in this row horizontally from left to right.
		ui.with_layout(egui::Layout::top_down_justified(egui::Align::TOP), |ui| {

			// Color picker!
			let mut rgb: [f32; 3] = [1.0, 0.0, 1.0];
			ui.color_edit_button_rgb(&mut rgb);

			// Tool size!
			let mut tool_size: f32 = 10.0;
			ui.add(egui::Slider::new(&mut tool_size, 1.0..=1000.0));
		});
    });

	// Scrub bar for time travel!
	egui::Window::new("Time Travel")
		.frame(window_frame)
		.fixed_pos(Pos2 { x: border_width, y: window_size.y } )
		.fixed_size(window_size)
		.show(contexts.ctx_mut(), |ui| {

		// Allow the UI windows to grow to the size of the screen.
		ui.set_width(window_size.x);
		ui.set_width(window_size.y);

		// Align the buttons in this row horizontally from left to right.
		ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {

			// Simulation play/pause.
			if ui.button("Play/Pause").clicked() {

			}

			// Time-travel using the scrub bar!
			let mut scrubbar: f32 = 0.0;
			ui.add(egui::Slider::new(&mut scrubbar, -120000.0..=0.0).text("Go back in time!")
				.show_value(false));
		});
    });
}
