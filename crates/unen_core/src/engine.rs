use unen_event::{EventHandler, EventManager};

use crate::event::EngineEvent;

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
    event_manager: EventManager,
}

impl Default for EngineData {
    fn default() -> Self {
        let event_manager = EventManager::default();

        Self {
            state: EngineState::Stopped,
            event_manager,
        }
    }
}

/// Represents the engine in the "stopped" state.
///
/// From here you can only call [`StoppedEngine::start`] to transition into a
/// [`StartedEngine`].
///
/// # Example
///
/// ```rust
/// use unen_core::prelude::*;
///
/// let engine = create_engine(); // initial state: stopped
/// let started = engine.start(); // transition to StartedEngine
/// ```
pub struct StoppedEngine {
    data: EngineData,
}

impl StoppedEngine {
    /// Starts the engine, consuming `self` and returning a [`StartedEngine`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use unen_core::prelude::*;
    ///
    /// let engine = create_engine();
    /// let started = engine.start(); // now the engine is started
    /// ```
    pub fn start(mut self) -> StartedEngine {
        self.data.state = EngineState::Started;
        self.data.event_manager.emit(EngineEvent::Starting);
        self.data.event_manager.emit(EngineEvent::Started);
        StartedEngine { data: self.data }
    }

    pub fn add_event_handler<H: EventHandler + 'static>(mut self, handler: H) -> Self {
        self.data.event_manager.add_handler(handler);
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
    #[allow(dead_code)]
    data: EngineData,
}

impl StartedEngine {
    /// Stops the engine, consuming `self` and returning a [`StoppedEngine`].
    ///
    /// # Example
    ///
    /// ```rust
    /// use unen_core::prelude::*;
    ///
    /// let engine = create_engine();
    /// let stopped = engine.start().stop(); // starts and stops the engine
    /// ```
    pub fn stop(mut self) -> StoppedEngine {
        self.data.event_manager.emit(EngineEvent::Stopping);
        self.data.event_manager.emit(EngineEvent::Stopped);
        StoppedEngine { data: self.data }
    }
}

/// Creates a new [`StoppedEngine`].
///
/// This is the recommended entry point to construct the engine. The initial
/// state will always be `Stopped`.
///
/// # Example
///
/// ```rust
/// use unen_core::prelude::*;
///
/// let engine = create_engine();
/// // still stopped, you need to call `.start()`
/// let engine = engine.start();
/// ```
pub fn create_engine() -> StoppedEngine {
    StoppedEngine {
        data: Default::default(),
    }
}
