use unen_event::prelude::{Event, EventEmitter, EventHandler, EventManager};

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
    fn run(&mut self, data: &RunnerData);
    fn step(&mut self, data: &RunnerData);
}

pub struct RunnerBox {
    runner: Box<dyn Runner>,
    data: RunnerData,
}

impl RunnerBox {
    pub fn new<R: Runner + 'static>(runner: R) -> Self {
        Self {
            runner: Box::new(runner),
            data: RunnerData::default(),
        }
    }

    pub fn run(&mut self) {
        self.runner.as_mut().run(&self.data)
    }

    pub fn step(&mut self) {
        self.data.event_manager.step();
        self.runner.as_mut().step(&self.data);
    }

    pub fn emit<E: Event>(&self, event: E) {
        self.data.event_emitter.emit(event);
    }

    pub fn add_event_handler<H: EventHandler + 'static>(&mut self, handler: H) {
        self.data.event_manager.add_handler(handler);
    }
}
