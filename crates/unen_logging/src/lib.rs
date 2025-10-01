use unen_core::prelude::EngineEvent;
use unen_event::EventHandler;

pub struct LoggerEventHandler;

impl EventHandler for LoggerEventHandler {
    fn handle(&mut self, event: &unen_event::EventBox) -> bool {
        if let Some(engine_event)= event.downcast_ref::<EngineEvent>() {
            match engine_event {
                EngineEvent::Starting => {
                    tracing_subscriber::fmt()
                        .with_max_level(tracing::Level::TRACE)
                        .with_target(false)
                        .with_level(true)
                        .init();
                    log::info!("UnnamedEngine is starting")
                }
                EngineEvent::Started => {
                    log::info!("UnnamedEngine successfully started");
                },
                EngineEvent::Stopping => {
                    log::info!("UnnamedEngine is stopping");
                }
                EngineEvent::Stopped => {
                    log::info!("UnnamedEngine successfully stopped");
                    log::info!("See you again :D")
                },
            }
        }

        false
    }
}

impl Default for LoggerEventHandler {
    fn default() -> Self {
        Self
    }
}
