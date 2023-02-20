use macros::*;
use serde::{Deserialize, Serialize};

#[test]
fn test_kind_error() {
    #[derive(KindError, Debug, PartialEq, Clone, Deserialize, Serialize)]
    enum ErrorKind {
        Red,
        Blue,
        Green,
    }
    assert_eq!(ErrorKind::Red.as_string(), String::from("Red"));
    assert_eq!(ErrorKind::Blue.as_string(), String::from("Blue"));
    assert_eq!(ErrorKind::Green.as_string(), String::from("Green"));
}
