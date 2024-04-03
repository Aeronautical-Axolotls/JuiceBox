use bevy::prelude::*;
use crate::ui::SimTool;

#[derive(Event)]
pub struct UseToolEvent {
    pub tool: SimTool,
    pub pos: Vec2,
    pub mouse_button: Option<MouseButton>,
}

impl UseToolEvent {
    pub fn new(tool: SimTool, pos: Vec2, mouse_button: Option<MouseButton>) -> Self {
       Self {
           tool,
           pos,
           mouse_button
       }
    }
}

#[derive(Event)]
pub struct ResetEvent;
