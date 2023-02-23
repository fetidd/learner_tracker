use lazy_static::lazy_static;
use regex::Regex;

pub fn is_valid_email(email: &str) -> bool {
    lazy_static! {
        static ref EMAIL_REGEX: Regex = Regex::new(r"[A-Za-z0-9._%+-]+@[A-Za-z0-9.-]+\.[A-Za-z]{2,}").unwrap();
    }
    EMAIL_REGEX.is_match(email)
}


