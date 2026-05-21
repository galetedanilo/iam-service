use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RegisterUserInput {
    email: String,
    password: String,
    correlation_id: Uuid,
}

impl RegisterUserInput {
    pub fn new(email: String, password: String, correlation_id: Uuid) -> Self {
        Self {
            email,
            password,
            correlation_id,
        }
    }

    pub fn email(&self) -> &str {
        &self.email
    }

    pub fn password(&self) -> &str {
        &self.password
    }

    pub fn correlation_id(&self) -> &Uuid {
        &self.correlation_id
    }
}
