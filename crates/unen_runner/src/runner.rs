use std::sync::{Arc, Mutex};

use unen_event::prelude::{Event, EventEmitter, EventHandler, EventManager};

pub type SharedRunnerData = Arc<Mutex<RunnerData>>;

pub struct RunnerData {
    pub event_manager: EventManager,
    pub event_emitter: EventEmitter,
}

impl Default for RunnerData {
    fn default() -> Self {
        let event_manager = EventManager::default();
        let event_emitter = event_manager.get_emitter();

        Self {
            event_manager,
            event_emitter,
        }
    }
}

pub trait Runner: Send + Sync {
    fn run(&mut self, data: SharedRunnerData);
}

pub struct RunnerBox {
    runner: Box<dyn Runner>,
    data: SharedRunnerData,
}

impl RunnerBox {
    pub fn new<R: Runner + 'static>(runner: R) -> Self {
        let data = Arc::new(Mutex::new(RunnerData::default()));
        Self {
            runner: Box::new(runner),
            data,
        }
    }

    pub fn run(&mut self) {
        self.runner.as_mut().run(Arc::clone(&self.data))
    }

    pub fn step(&mut self) {
        self.data.lock().unwrap().event_manager.step();
    }

    pub fn emit<E: Event>(&self, event: E) {
        self.data.lock().unwrap().event_emitter.emit(event);
    }

    pub fn add_event_handler<H: EventHandler + 'static>(&mut self, handler: H) {
        self.data.lock().unwrap().event_manager.add_handler(handler);
    }
}
