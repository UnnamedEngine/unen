use crate::{Event, EventBox, EventHandler};

pub struct EventManager {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl EventManager {
    pub fn emit<E: Event>(&mut self, event: E) {
        let event_box = EventBox::new(event);
        for handler in &mut self.handlers {
            if handler.handle(&event_box) {
                break;
            }
        }
    }

    pub fn add_handler<H: EventHandler + 'static>(&mut self, handler: H) {
        self.handlers.push(Box::new(handler));
    }
}

impl Default for EventManager {
    fn default() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }
}
