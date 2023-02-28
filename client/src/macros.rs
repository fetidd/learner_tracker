#[macro_export]
macro_rules! log {
    ($($arg:expr),+) => {
        gloo_console::log!("LEARNER TRACKER| ", $($arg),+)
    }
}

#[macro_export]
macro_rules! debug {
    ($($arg:expr),+) => {
        gloo_console::debug!("LEARNER TRACKER| ", $($arg),+)
    }
}

#[macro_export]
macro_rules! error {
    ($($arg:expr),+) => {
        gloo_console::error!("LEARNER TRACKER| ", $($arg),+)
    }
}

#[macro_export]
macro_rules! from_error {
    ($error:ty > $kind:ident) => {
        impl From<$error> for Error {
            fn from(value: $error) -> Self {
                Error {
                    kind: ErrorKind::$kind,
                    details: Some(format!("{value:?}")),
                }
            }
        }
    };
    ($error:ty > $kind:ident: $msg:expr) => {
        impl From<$error> for Error {
            fn from(_value: $error) -> Self {
                Error {
                    kind: ErrorKind::$kind,
                    details: Some($msg.to_string()),
                }
            }
        }
    };
}

#[macro_export]
macro_rules! error_macro {
    ($($kind:ident),+) => {
        $(macro_rules! $kind {
            () => {{
                let e = crate::error::Error {
                    kind: crate::error::ErrorKind::$kind,
                    details: None,
                };
                error!("{}", e.to_string());
                e
            }};
            ($msg:expr) => {{
                let e = crate::error::Error {
                    kind: crate::error::ErrorKind::$kind,
                    details: Some(String::from($msg)),
                };
                error!("{}", e.to_string());
                e
            }};
        })+
    };
}

