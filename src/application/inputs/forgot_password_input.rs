use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct ForgotPasswordInput {
    email: String,
    correlation_id: Uuid,
}

impl ForgotPasswordInput {
    pub fn new(email: String, correlation_id: Uuid) -> Self {
        Self {
            email,
            correlation_id,
        }
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn correlation_id(&self) -> &Uuid {
        &self.correlation_id
    }
}
