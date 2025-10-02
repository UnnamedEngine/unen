use unen_event::prelude::Event;

#[derive(Debug, Event)]
pub enum RunnerEvent {
    Step,
    Terminate,
}
