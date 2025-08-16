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
        // The logger is started here to make sure we have logging from the
        // start
        // Another important thing is that we DON'T want logging for tests
        #[cfg(not(test))]
        {
            use crate::core::logging;

            logging::setup_logger();
        }

        Self {
            state: EngineState::Stopped,
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
/// use unen::core::engine::create_engine;
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
    /// use unen::core::engine::create_engine;
    ///
    /// let engine = create_engine();
    /// let started = engine.start(); // now the engine is started
    /// ```
    pub fn start(mut self) -> StartedEngine {
        self.data.state = EngineState::Started;
        StartedEngine { data: self.data }
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
}

/// Creates a new [`StoppedEngine`].
///
/// This is the recommended entry point to construct the engine. The initial
/// state will always be `Stopped`.
///
/// # Example
///
/// ```rust
/// use unen::core::engine::create_engine;
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
