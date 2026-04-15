use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, EnumString, Display, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum OutboxStatus {
    Pending,
    Processing,
    Processed,
}
