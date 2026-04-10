use strum_macros::{AsRefStr, Display, EnumString};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, AsRefStr, Display, EnumString)]
pub enum IamScope {
    #[strum(serialize = "iam:admin")]
    Admin,
    #[strum(serialize = "iam:create")]
    Create,
    #[strum(serialize = "iam:delete")]
    Delete,
    #[strum(serialize = "iam:read")]
    Read,
    #[strum(serialize = "iam:update")]
    Update,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, AsRefStr, Display, EnumString)]
pub enum ProfileScope {
    #[strum(serialize = "profile:admin")]
    Admin,
    #[strum(serialize = "profile:create")]
    Create,
    #[strum(serialize = "profile:delete")]
    Delete,
    #[strum(serialize = "profile:read")]
    Read,
    #[strum(serialize = "profile:update")]
    Update,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, AsRefStr, Display)]
pub enum Scope {
    IamService(IamScope),
    ProfileService(ProfileScope),
}

impl Default for Scope {
    fn default() -> Self {
        Scope::ProfileService(ProfileScope::Read)
    }
}
