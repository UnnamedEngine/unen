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

use crate::{
    prelude::RunnerEvent,
    runner::{Runner, RunnerData},
};

pub struct MininalRunner {
    term: Arc<AtomicBool>,
}

impl Runner for MininalRunner {
    fn run(&mut self, data: &RunnerData) {
        let _ = flag::register(SIGINT, Arc::clone(&self.term));
        let _ = flag::register(SIGTERM, Arc::clone(&self.term));

        while !self.term.load(Ordering::Relaxed) {
            self.step(data);
            thread::sleep(Duration::from_millis(1));
        }

        // Prints a newline to not mix logs with ctl echo
        println!();

        data.event_emitter.emit(RunnerEvent::Terminate);
    }

    fn step(&mut self, data: &RunnerData) {
        data.event_emitter.emit(RunnerEvent::Step);
    }
}

impl Default for MininalRunner {
    fn default() -> Self {
        Self {
            term: Arc::new(AtomicBool::new(false)),
        }
    }
}
