use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, EnumString, Display, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum Status {
    Active,
    PendingConfirmation,
    Suspended,
    Banned,
    Deleted,
}

impl Status {
    pub fn can_authenticate(&self) -> bool {
        match self {
            Status::Active => true,
            _ => false,
        }
    }

    pub fn is_recoverable(&self) -> bool {
        matches!(self, Status::Suspended | Status::Deleted)
    }

    pub fn is_confirmed(&self) -> bool {
        matches!(self, Status::Active)
    }
}
