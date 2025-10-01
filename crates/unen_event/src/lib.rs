mod event;
mod handler;
mod manager;

pub use crate::{event::Event, event::EventBox, handler::EventHandler, manager::EventManager};
#[cfg(feature = "derive")]
pub use unen_event_derive::Event;

/// The prelude.
pub mod prelude {
    pub use crate::{event::Event, event::EventBox, handler::EventHandler, manager::EventManager};
    #[cfg(feature = "derive")]
    pub use unen_event_derive::Event;
}
