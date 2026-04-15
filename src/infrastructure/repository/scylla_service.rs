use scylla::client::{session::Session, session_builder::SessionBuilder};
use scylla_migrate::Migrator;

pub struct ScyllaService {}

impl ScyllaService {
    pub async fn new(
        hostnames: Vec<&str>,
        keyspace_name: &str,
        username: &str,
        password: &str,
        case_sensitive: bool,
    ) -> anyhow::Result<Session> {
        let session = SessionBuilder::new()
            .known_nodes(hostnames)
            .user(username, password)
            .use_keyspace(keyspace_name, case_sensitive)
            .build()
            .await?;

        let runner = Migrator::new(&session, "./migrations");

        runner.run().await?;

        Ok(session)
    }
}
