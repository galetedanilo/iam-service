use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, AsRefStr, Display, EnumString)]
pub enum Scope {
    // IAM Scopes
    #[strum(serialize = "iam:admin")]
    IamAdmin,
    #[strum(serialize = "iam:create")]
    IamCreate,
    #[strum(serialize = "iam:delete")]
    IamDelete,
    #[strum(serialize = "iam:read")]
    IamRead,
    #[strum(serialize = "iam:update")]
    IamUpdate,

    // Profile Scopes
    #[strum(serialize = "profile:admin")]
    ProfileAdmin,
    #[strum(serialize = "profile:create")]
    ProfileCreate,
    #[strum(serialize = "profile:delete")]
    ProfileDelete,
    #[strum(serialize = "profile:read")]
    ProfileRead,
    #[strum(serialize = "profile:update")]
    ProfileUpdate,
}

impl Default for Scope {
    fn default() -> Self {
        Scope::ProfileRead
    }
}
