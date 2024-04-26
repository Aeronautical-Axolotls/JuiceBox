use bevy::prelude::*;
use bevy::ecs::event::Event;
use crate::juice_renderer::FluidColorRenderType;
use crate::ui::{SimTool, UIStateManager};

/**
  Use tool event that sends
  the event to be handled by the simulation
  state manager
*/
#[derive(Event)]
pub struct UseToolEvent {
    pub tool: SimTool,                      // Tool Used
    pub pos: Vec2,                          // Mouse position
    pub mouse_button: Option<MouseButton>,  // Mouse button pressed
	pub mouse_held: bool,					// Is the mouse being held or has it just been pressed?
}

impl UseToolEvent {

    /// New function for creating new events
    pub fn new(tool: SimTool, pos: Vec2, mouse_button: Option<MouseButton>, mouse_held: bool) -> Self {
       Self {
           tool,
           pos,
           mouse_button,
		   mouse_held,
       }
    }
}

/**
    Reset event for reseting the simulation.
    Handled by the simulation state manager
*/
#[derive(Event)]
pub struct ResetEvent;

/** Event that controls play/pause/stepping.  Here is how it works:
	- `is_step_event == true`, `sim_paused == true`: Simulation steps one time.
	- `is_step_event == true`, `sim_paused == false`: Simulation pauses and steps one time.
	- `is_step_event == false`, `sim_paused == true`: Simulation unpauses.
	- `is_step_event == false`, `sim_paused == false`: Simulation pauses. */
#[derive(Event)]
pub struct PlayPauseStepEvent {
	pub is_step_event: bool,
}

// Create a new play/pause/step event.
impl PlayPauseStepEvent {
	pub fn new(is_step_event: bool) -> Self {
		Self {
			is_step_event: is_step_event,
		}
	}
}

#[derive(Event)]
pub struct ModifyVisualizationEvent {
	pub show_grid:			bool,
	pub show_velocities:	bool,
	pub show_gravity:		bool,

	pub color_variable:		FluidColorRenderType,
	pub fluid_colors:		[[f32; 3]; 4],
	pub particle_size:		f32,
}

/* Create a new visualization modification event, copying the appropriate parameters from the UI
	state manager.  Is this adaptable to other systems?  No!  But it will look nice when it is
	called and it will do exactly what we need it to do for this system. */
impl ModifyVisualizationEvent {
    pub fn new(ui_state: &UIStateManager) -> Self {

		let fluid_color_variable: FluidColorRenderType = match ui_state.fluid_color_variable {
			0 => { FluidColorRenderType::Velocity },
			1 => { FluidColorRenderType::Density },
			2 => { FluidColorRenderType::Pressure },
			_ => { FluidColorRenderType::Arbitrary },
		};

    	Self {
			show_grid:			ui_state.show_grid,
			show_velocities:	ui_state.show_velocity_vectors,
			show_gravity:		ui_state.show_gravity_vector,
			color_variable:		fluid_color_variable,
			fluid_colors:		ui_state.fluid_colors,
			particle_size:		ui_state.particle_physical_size,
    	}
    }
}