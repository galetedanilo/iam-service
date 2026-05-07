use std::sync::Arc;

use chrono::{DateTime, Utc};
use scylla::{
    client::session::Session,
    errors::{ExecutionError, FirstRowError, IntoRowsResultError, MaybeFirstRowError},
    statement::batch::Batch,
};
use std::str::FromStr;
use uuid::Uuid;

use crate::{
    domain::{
        enums::{audience::Audience, scope::Scope},
        events::event::{Event, EventPayload},
        models::user::User,
        object_values::{email::Email, id::Id, password::Password, status::Status},
        repositories::user_repository::{UserRepository, UserRepositoryError},
    },
    infrastructure::repository::outbox::OutboxStatus,
};

#[derive(Clone)]
pub struct ScyllaUserRepository {
    session: Arc<Session>,
}

impl ScyllaUserRepository {
    pub fn new(session: Arc<Session>) -> Self {
        Self { session }
    }
}

#[async_trait::async_trait]
impl UserRepository for ScyllaUserRepository {
    #[tracing::instrument(name = "Saving user to ScyllaDB", skip(self, user, event))]
    async fn save<T>(&self, user: &User, event: &Event<T>) -> Result<(), UserRepositoryError>
    where
        T: EventPayload,
    {
        // 1. Definição do Bucket para o Outbox (ex: 10 shards por dia)
        let shard_id = rand::random::<u8>() % 10;
        let bucket_id = format!("outbox-{}", shard_id);

        // 2. RESERVA DE E-MAIL (LWT)
        // Isso garante que ninguém mais use este e-mail
        let reserve_query = "INSERT INTO email_lookup (email, user_id) VALUES (?, ?) IF NOT EXISTS";
        let _ = self
            .session
            .query_unpaged(reserve_query, (user.email().as_ref(), user.id().as_ref()))
            .await?
            .into_rows_result()?;

        let mut batch = Batch::default();

        // 3. BATCH ATÔMICO (Usuário + Outbox )
        // Se um falhar, o outro não é gravado
        batch.append_statement("INSERT INTO users (id, email, password, audiences, scopes, status, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?, ?, ?)");

        // 4. Query para a Outbox Genérica
        batch.append_statement(
            "INSERT INTO outbox (bucket_id, event_id, status, lease_expires, payload, metadata, event_type, occurred_at, exchange_name) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)",
        );

        // 5. Valores para o batch (ordem deve corresponder às queries acima)
        let batch_values = (
            (
                user.id().as_ref(),
                user.email().as_ref(),
                user.password().as_ref(),
                user.audiences()
                    .iter()
                    .map(|a| a.as_ref())
                    .collect::<Vec<&str>>(),
                user.scopes()
                    .iter()
                    .map(|s| s.as_ref())
                    .collect::<Vec<&str>>(),
                user.status().as_ref(),
                user.created_at(),
                user.updated_at(),
            ), // Dados do User
            (
                bucket_id,
                event.id(),
                OutboxStatus::Pending.as_ref(),
                Utc::now(),
                event.build_payload_json(),
                event.metadata(),
                event.event_type().as_ref(),
                event.occurred_at(),
                event.exchange_name(),
            ), // Dados da Outbox
        );

        // 6. Execução do batch
        if let Err(e) = self.session.batch(&batch, batch_values).await {
            // Rollback manual da reserva de e-mail caso o batch falhe drasticamente
            self.session
                .query_unpaged(
                    "DELETE FROM email_lookup WHERE email = ?",
                    (user.email().as_ref(),),
                )
                .await?;

            return Err(UserRepositoryError::from(e));
        }

        Ok(())
    }

    #[tracing::instrument(name = "Finding user by email in ScyllaDB", skip(self, email))]
    async fn find_by_email(&self, email: &Email) -> Result<Option<User>, UserRepositoryError> {
        let rows = self
            .session
            .query_unpaged(
                "SELECT email, user_id FROM email_lookup WHERE email = ?",
                (email.as_ref(),),
            )
            .await?
            .into_rows_result()?;

        let first_result = match rows.first_row::<(String, Uuid)>() {
            Ok(row) => row,
            Err(FirstRowError::RowsEmpty) => return Ok(None),
            Err(e) => return Err(UserRepositoryError::Unknown(e.to_string())),
        };

        let (_, user_id) = first_result;
        let id = Id::from_uuid(user_id);

        self.find_by_id(&id).await
    }

    #[tracing::instrument(name = "Finding user by ID in ScyllaDB", skip(self, id))]
    async fn find_by_id(&self, id: &Id) -> Result<Option<User>, UserRepositoryError> {
        let rows = self
            .session
            .query_unpaged(
                "SELECT id, email, password, status, audiences, scopes, created_at, updated_at FROM users WHERE id = ?",
                (id.as_ref(),),
            )
            .await?
            .into_rows_result()?;

        let first_result = match rows.first_row::<(
            Uuid,
            String,
            String,
            String,
            Vec<String>,
            Vec<String>,
            DateTime<Utc>,
            DateTime<Utc>,
        )>() {
            Ok(row) => row,
            Err(FirstRowError::RowsEmpty) => return Ok(None),
            Err(e) => return Err(UserRepositoryError::Unknown(e.to_string())),
        };

        let (id, email_str, password_str, status, audiences, scopes, created_at, updated_at) =
            first_result;

        let user = User::from_parts(
            Id::from_uuid(id),
            Email::try_new(email_str)
                .map_err(|e| UserRepositoryError::InvalidData(e.to_string()))?,
            Password::from_hash(password_str),
            Status::from_str(&status)
                .map_err(|e| UserRepositoryError::InvalidData(e.to_string()))?,
            audiences
                .into_iter()
                .filter_map(|s| Audience::from_str(&s).ok())
                .collect(),
            scopes
                .into_iter()
                .filter_map(|s| Scope::from_str(&s).ok())
                .collect(),
            created_at,
            updated_at,
        );

        Ok(Some(user))
    }
}

impl From<ExecutionError> for UserRepositoryError {
    fn from(value: ExecutionError) -> Self {
        tracing::error!("Execution error: {}", value);
        UserRepositoryError::Unknown(value.to_string())
    }
}

impl From<IntoRowsResultError> for UserRepositoryError {
    fn from(value: IntoRowsResultError) -> Self {
        tracing::error!("Into rows result error: {}", value);
        UserRepositoryError::Unknown(value.to_string())
    }
}

impl From<FirstRowError> for UserRepositoryError {
    fn from(value: FirstRowError) -> Self {
        tracing::error!("First row error: {}", value);
        UserRepositoryError::Unknown(value.to_string())
    }
}

impl From<MaybeFirstRowError> for UserRepositoryError {
    fn from(value: MaybeFirstRowError) -> Self {
        tracing::error!("Maybe first row error: {}", value);
        UserRepositoryError::Unknown(value.to_string())
    }
}
