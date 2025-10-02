mod engine_event;
mod event;
mod handler;
mod manager;

/// The prelude.
pub mod prelude {
    pub use crate::{
        engine_event::EngineEvent, event::Event, event::EventBox, handler::EventHandler,
        manager::EventEmitter, manager::EventManager,
    };
    #[cfg(feature = "derive")]
    pub use unen_event_derive::Event;
}
