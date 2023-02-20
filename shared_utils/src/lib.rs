use lazy_static::lazy_static;
use regex::Regex;

pub fn is_valid_email(email: &str) -> bool {
    lazy_static! {
        static ref EMAIL_REGEX: Regex =
            Regex::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}").unwrap();
    }
    EMAIL_REGEX.is_match(email)
}

#[macro_export]
macro_rules! from_error {
    ($error:ty > $kind:ident) => {
        impl From<$error> for Error {
            fn from(value: $error) -> Self {
                Error {
                    kind: ErrorKind::$kind,
                    message: Some(value.to_string()),
                }
            }
        }
    };
    ($error:ty > $kind:ident: $msg:expr) => {
        impl From<$error> for Error {
            fn from(_value: $error) -> Self {
                Error {
                    kind: ErrorKind::$kind,
                    message: Some($msg.to_string()),
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
                let e = Error {
                    kind: ErrorKind::$kind,
                    message: None,
                };
                tracing::error!("{}", e.to_string());
                e
            }};
            ($msg:expr) => {{
                let e = Error {
                    kind: ErrorKind::$kind,
                    message: Some(String::from($msg)),
                };
                tracing::error!("{}", e.to_string());
                e
            }};
        })+
    };
}
