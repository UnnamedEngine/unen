mod handle;
mod window_event;

pub mod prelude {
    pub use crate::{handle::SendableWindowHandle, window_event::WindowEvent};
}
