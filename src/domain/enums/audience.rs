use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, EnumString, Display, AsRefStr)]
pub enum Audience {
    #[strum(serialize = "iam-service")]
    IamService,

    #[strum(serialize = "profile-service")]
    ProfileService,
}

impl Default for Audience {
    fn default() -> Self {
        Audience::ProfileService
    }
}
