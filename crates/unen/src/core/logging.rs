#[allow(dead_code)]
pub fn setup_logger() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .with_target(false)
        .with_level(true)
        .init();
}
