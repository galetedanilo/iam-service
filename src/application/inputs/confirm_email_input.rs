#[derive(Debug, Clone)]
pub struct ConfirmEmailInput {
    pub jwt: String,
}

impl ConfirmEmailInput {
    pub fn new(jwt: String) -> Self {
        Self { jwt }
    }
}
