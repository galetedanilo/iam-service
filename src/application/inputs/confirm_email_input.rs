use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ConfirmEmailInput {
    jwt: String,
    correlation_id: Uuid,
}

impl ConfirmEmailInput {
    pub fn new(jwt: String, correlation_id: Uuid) -> Self {
        Self {
            jwt,
            correlation_id,
        }
    }

    pub fn jwt(&self) -> &str {
        &self.jwt
    }
    
    pub fn correlation_id(&self) -> &Uuid {
        &self.correlation_id
    }
}
