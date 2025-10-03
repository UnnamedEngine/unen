use crate::prelude::Event;

#[derive(Debug, Event)]
pub enum EngineEvent {
    Starting,
    Started,
    Update,
    Stopping,
    Stopped,
}
