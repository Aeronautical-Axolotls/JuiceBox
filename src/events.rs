use bevy::prelude::*;
use bevy::ecs::event::Event;
use crate::ui::SimTool;

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
}

impl UseToolEvent {

    /// New function for creating new events
    pub fn new(tool: SimTool, pos: Vec2, mouse_button: Option<MouseButton>) -> Self {
       Self {
           tool,
           pos,
           mouse_button
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
	is_step_event: bool,
}