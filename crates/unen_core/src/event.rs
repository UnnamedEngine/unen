use unen_event::Event;

#[derive(Debug, Event)]
pub enum EngineEvent {
    Starting,
    Started,
    Stopping,
    Stopped,
}
