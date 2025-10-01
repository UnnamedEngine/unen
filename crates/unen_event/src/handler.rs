use crate::EventBox;

pub trait EventHandler: Send + Sync {
    fn handle(&mut self, event: &EventBox) -> bool;
}
