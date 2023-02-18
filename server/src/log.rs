use tracing_subscriber::{fmt, EnvFilter};

pub fn start_log() {
    let log_fmt = fmt::format() // TODO move to log module
        .compact()
        .without_time()
        .with_target(true)
        .with_thread_names(false);
    fmt()
        .event_format(log_fmt)
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}
