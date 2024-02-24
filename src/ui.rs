use bevy::{ecs::system::Query, window::Window};
use bevy_egui::{egui::{self, Pos2, Vec2},EguiContexts};

pub fn ui_base(mut contexts: EguiContexts, windows: Query<&Window>) {

	let window = windows.single();
	let window_size: Vec2 = Vec2 { x: window.width(), y: window.height() };

	egui::Window::new("Scene Manager")
		.fixed_pos(Pos2 { x: 0.0, y: 0.0 } )
		.fixed_size(window_size)
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

	egui::Window::new("Time Travel")
		.fixed_pos(Pos2 { x: 0.0, y: window_size.y } )
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
