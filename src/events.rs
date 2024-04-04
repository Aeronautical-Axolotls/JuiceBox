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

/** Event that sends camera controller impulses.  Fields are relative; `z_rotation` represents the
	change in rotation to apply, not the camera's new rotation value. */
#[derive(Event)]
pub struct CameraEvent {
	pub z_rotation:		f32,
	pub translation:	Vec2,
	pub zoom:			f32,
}

/** Event that sends gravity change events.  Fields are relative; `rotation_direction` represents
	whether to rotate left, right, or not at all (-1, 1, 0). */
#[derive(Event)]
pub struct GravityChangeEvent {
	pub direction:	i8,
	pub magnitude:	i8,
}

// Create a new event for sending through Bevy's EventWriters!
impl GravityChangeEvent {
    pub fn new(rotation_direction_sign: i8, magnitude_change_sign: i8) -> Self {
       Self {
           direction:	rotation_direction_sign,
		   magnitude:	magnitude_change_sign,
       }
    }
}