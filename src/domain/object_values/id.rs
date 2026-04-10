use std::fmt::Display;

use thiserror::Error;
use uuid::Uuid;

use crate::domain::models::user::UserError;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Id(Uuid);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Error)]
pub enum IdError {
    #[error("Invalid id: {0}")]
    Invalid(String),
}

impl Id {
    pub fn generate() -> Self {
        Self(Uuid::now_v7())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        Self(uuid)
    }

    pub fn into_inner(self) -> Uuid {
        self.0
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<Uuid> for Id {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl TryFrom<&str> for Id {
    type Error = IdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Uuid::parse_str(value)
            .map_err(|e| IdError::Invalid(e.to_string()))
            .map(Id)
    }
}

impl TryFrom<String> for Id {
    type Error = IdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Uuid::parse_str(&value)
            .map_err(|e| IdError::Invalid(e.to_string()))
            .map(Id)
    }
}

impl From<IdError> for UserError {
    fn from(error: IdError) -> Self {
        UserError::InvalidData(error.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_generation() {
        let id1 = Id::generate();
        let id2 = Id::generate();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_id_from_uuid() {
        let uuid = Uuid::now_v7();
        let id = Id::from_uuid(uuid);
        assert_eq!(id.into_inner(), uuid);
    }

    #[test]
    fn test_id_try_from_str_valid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000";
        let id = Id::try_from(uuid_str).unwrap();
        assert_eq!(id.into_inner().to_string(), uuid_str);
    }

    #[test]
    fn test_id_try_from_str_invalid() {
        let invalid_uuid_str = "invalid-uuid";
        let result = Id::try_from(invalid_uuid_str);
        assert!(result.is_err());
    }

    #[test]
    fn test_id_try_from_string_valid() {
        let uuid_str = "550e8400-e29b-41d4-a716-446655440000".to_string();
        let id = Id::try_from(uuid_str.clone()).unwrap();
        assert_eq!(id.into_inner().to_string(), uuid_str);
    }

    #[test]
    fn test_id_try_from_string_invalid() {
        let invalid_uuid_str = "invalid-uuid".to_string();
        let result = Id::try_from(invalid_uuid_str);
        assert!(result.is_err());
    }
}
