use unen::predlue::create_engine;
use unen_logging::LoggerEventHandler;

fn main() {
    let _ = create_engine()
        .add_event_handler(LoggerEventHandler::default())
        .start()
        .stop();
}
