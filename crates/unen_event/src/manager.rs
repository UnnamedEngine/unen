use std::sync::mpsc::{channel, Receiver, Sender};

use crate::{
    event::Event,
    prelude::{EventBox, EventHandler},
};

type EventReceiver = Receiver<EventBox>;

#[derive(Debug, Clone)]
pub struct EventEmitter {
    sender: Sender<EventBox>,
}

impl EventEmitter {
    pub fn emit<E: Event>(&self, event: E) {
        let _ = self.sender.send(EventBox::new(event));
    }
}

pub struct EventManager {
    handlers: Vec<Box<dyn EventHandler>>,
    receiver: EventReceiver,
    emitter: EventEmitter,
}

impl EventManager {
    pub fn add_handler<H: EventHandler + 'static>(&mut self, handler: H) {
        self.handlers.push(Box::new(handler));
    }

    pub fn step(&mut self) {
        while let Ok(event) = self.receiver.try_recv() {
            for handler in &mut self.handlers {
                if handler.handle(&event) {
                    break;
                }
            }
        }
    }

    pub fn get_emitter(&self) -> EventEmitter {
        self.emitter.clone()
    }
}

impl Default for EventManager {
    fn default() -> Self {
        let (sender, receiver) = channel();
        let emitter = EventEmitter { sender };

        Self {
            handlers: Vec::new(),
            receiver,
            emitter,
        }
    }
}
