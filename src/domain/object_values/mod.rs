use lazy_static::lazy_static;
use regex::Regex;

pub mod email;
pub mod id;
pub mod password;
pub mod status;

lazy_static! {
    pub static ref EMAIL_REGEX: Regex =
        Regex::new(r"(?i)^[a-z0-9._%+-]+@[a-z0-9.-]+\.[a-z]{2,}$").unwrap();
}
