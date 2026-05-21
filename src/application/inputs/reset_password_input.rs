use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ResetPasswordInput {
    jwt: String,
    new_password: String,
    correlation_id: Uuid,
}

impl ResetPasswordInput {
    pub fn new(jwt: String, new_password: String, correlation_id: Uuid) -> Self {
        Self {
            jwt,
            new_password,
            correlation_id,
        }
    }

    pub fn jwt(&self) -> &str {
        &self.jwt
    }

    pub fn new_password(&self) -> &str {
        &self.new_password
    }

    pub fn correlation_id(&self) -> &Uuid {
        &self.correlation_id
    }
}
