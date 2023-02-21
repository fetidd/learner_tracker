#[macro_export]
macro_rules! log {
    ($($arg:expr),+) => {
        gloo_console::log!(constant::LOG_PREFIX, $($arg),+)
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:expr),+) => {
        gloo_console::debug!(constant::LOG_PREFIX, $($arg),+)
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:expr),+) => {
        gloo_console::error!(constant::LOG_PREFIX, $($arg),+)
    }
}
