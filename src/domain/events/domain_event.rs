use strum_macros::{AsRefStr, Display, EnumString};

pub trait DomainEvent: Send + Sync {
    fn id(&self) -> uuid::Uuid;
    
    fn exchange_name(&self) -> String;

    fn raw_event(&self) -> String;

    fn routing_key(&self) -> String;
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, EnumString, Display, AsRefStr)]
#[strum(serialize_all = "SCREAMING_SNAKE_CASE")]
pub enum EventType {
    UserRegistered,
    UserActivated,
    PasswordResetRequested,
    PasswordResetCompleted,
    // Outros tipos de eventos podem ser adicionados aqui
}
