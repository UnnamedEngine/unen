use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

#[derive(Debug, Clone, Copy)]
pub struct SendableWindowHandle {
    window: RawWindowHandle,
    display: RawDisplayHandle,
}

unsafe impl Send for SendableWindowHandle {}
unsafe impl Sync for SendableWindowHandle {}

impl SendableWindowHandle {
    pub fn new(window: RawWindowHandle, display: RawDisplayHandle) -> Self {
        Self { window, display }
    }

    pub fn window_handle(&self) -> RawWindowHandle {
        self.window
    }

    pub fn display_handle(&self) -> RawDisplayHandle {
        self.display
    }
}
