use macros::*;
use serde::{Deserialize, Serialize};

#[test]
fn test_kind_error() {
    #[derive(KindError, Debug, PartialEq, Clone, Deserialize, Serialize)]
    pub enum ErrorKind {
        Red,
        BlueGreen,
        Green,
    }
    assert_eq!(ErrorKind::Red.to_string(), String::from("RED"));
    assert_eq!(ErrorKind::BlueGreen.to_string(), String::from("BLUE GREEN"));
    assert_eq!(ErrorKind::Green.to_string(), String::from("GREEN"));


}
