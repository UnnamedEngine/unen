use unen_event::prelude::{EngineEvent, EventHandler};
use unen_runner::prelude::{MininalRunner, Runner, RunnerBox};

/// Represents the possible states of the engine.
///
/// Noramlly not used directly by the user.
/// State transitions are handled via [`StoppedEngine`] and [`StartedEngine`].
#[derive(Debug, PartialEq, Eq)]
enum EngineState {
    Stopped,
    Started,
}

/// Internal structure holding common engine data, including the current state.
///
/// End users usually don't interact with this directly — use [`StoppedEngine`]
/// or [`StartedEngine`] instead.
struct EngineData {
    state: EngineState,
}

impl Default for EngineData {
    fn default() -> Self {
        Self {
            state: EngineState::Stopped,
        }
    }
}

/// Represents the engine in the "stopped" state.
///
/// From here you can only call [`StoppedEngine::start`] to transition into a
/// [`StartedEngine`].
pub struct StoppedEngine {
    data: EngineData,
    runner: RunnerBox,
}

impl StoppedEngine {
    /// Starts the engine, consuming `self` and returning a [`StartedEngine`].
    pub fn start(mut self) -> StartedEngine {
        self.data.state = EngineState::Started;
        // We must step since there is no runner yet
        self.runner.emit(EngineEvent::Starting);
        self.runner.step();

        // We must step since there is no runner yet
        self.runner.emit(EngineEvent::Started);
        self.runner.step();

        self.runner.run();

        StartedEngine {
            data: self.data,
            runner: self.runner,
        }
    }

    pub fn add_event_handler<H: EventHandler + 'static>(mut self, handler: H) -> Self {
        self.runner.add_event_handler(handler);
        self
    }

    pub fn set_runner<R: Runner + 'static>(mut self, runner: R) -> Self {
        self.runner = RunnerBox::new(runner);
        self
    }
}

/// Represents the engine in the "started" state.
///
/// This type usually never appears in real code flow, since calling
/// [`StoppedEngine::start`] is expected to enter the main loop and **not
/// return** under normal circumstances.
///
/// The existence of this type is only to model state transitions at the type
/// level, making invalid states unrepresentable.
pub struct StartedEngine {
    data: EngineData,
    runner: RunnerBox,
}

impl StartedEngine {
    /// Stops the engine, consuming `self` and returning a [`StoppedEngine`].
    pub fn stop(mut self) -> StoppedEngine {
        // We must step since there is no runner anymore
        self.runner.emit(EngineEvent::Stopping);
        self.runner.step();

        // We must step since there is no runner anymore
        self.runner.emit(EngineEvent::Stopped);
        self.runner.step();

        StoppedEngine {
            data: self.data,
            runner: self.runner,
        }
    }
}

/// Creates a new [`StoppedEngine`].
///
/// This is the recommended entry point to construct the engine. The initial
/// state will always be `Stopped`.
pub fn create_engine() -> StoppedEngine {
    StoppedEngine {
        data: Default::default(),
        runner: RunnerBox::new(MininalRunner::default()),
    }
}
