use unen_event::prelude::Event;

use crate::handle::SendableWindowHandle;

#[derive(Debug, Event)]
pub enum WindowEvent {
    Created(SendableWindowHandle),
    Resized { width: u32, height: u32 },
    Redraw,
    Destroyed,
}
