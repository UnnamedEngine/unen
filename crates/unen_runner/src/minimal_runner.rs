use std::{
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    thread,
    time::Duration,
};

use signal_hook::{
    consts::{SIGINT, SIGTERM},
    flag,
};
use unen_event::prelude::EngineEvent;

use crate::runner::{Runner, SharedRunnerData};

pub struct MininalRunner {
    term: Arc<AtomicBool>,
}

impl Runner for MininalRunner {
    fn run(&mut self, data: SharedRunnerData) {
        let _ = flag::register(SIGINT, Arc::clone(&self.term));
        let _ = flag::register(SIGTERM, Arc::clone(&self.term));

        while !self.term.load(Ordering::Relaxed) {
            data.lock().unwrap().event_emitter.emit(EngineEvent::Update);
            data.lock().unwrap().event_manager.step();
            thread::sleep(Duration::from_millis(1));
        }

        // Prints a newline to not mix logs with ctl echo
        println!();
    }
}

impl Default for MininalRunner {
    fn default() -> Self {
        Self {
            term: Arc::new(AtomicBool::new(false)),
        }
    }
}
