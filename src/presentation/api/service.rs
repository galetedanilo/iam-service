use std::{net::SocketAddr, sync::Arc};

use axum::{
    Router,
    http::{
        HeaderValue, Method,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    routing::{get, post},
};
use jsonwebtoken::EncodingKey;
use tower_governor::{GovernorLayer, governor::GovernorConfigBuilder};
use tower_http::{cors::CorsLayer, trace::TraceLayer};

use crate::{
    infrastructure::repository::{
        scylla_service::ScyllaService, scylla_user_repo::ScyllaUserRepository,
    },
    presentation::api::{
        handlers::{
            confirm_email_handler::confirm_email_handler,
            register_user_handler::register_user_handler,
        },
        helpers::{app_state::AppState, config::Config, telemetry_config::init_telemetry},
    },
};

pub struct Service {
    config: Config,
}

impl Service {
    pub fn start() -> Self {
        let config = Config::new(
            std::env::var("REQUEST_HOST").expect("REQUEST_HOST must be set"),
            std::env::var("SERVICE_ADDR").expect("SERVICE_ADDR must be set"),
            std::env::var("PRIVATE_KEY_PATH").expect("PRIVATE_KEY_PATH must be set"),
            std::env::var("SCYLLA_HOSTNAMES")
                .expect("SCYLLA_HOSTNAMES must be set")
                .split(',')
                .map(String::from)
                .collect(),
            std::env::var("SCYLLA_KEYSPACE_NAME").expect("SCYLLA_KEYSPACE_NAME must be set"),
            std::env::var("SCYLLA_USERNAME").expect("SCYLLA_USERNAME must be set"),
            std::env::var("SCYLLA_PASSWORD").expect("SCYLLA_PASSWORD must be set"),
            std::env::var("SCYLLA_CASE_SENSITIVE")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .expect("SCYLLA_CASE_SENSITIVE must be a boolean"),
        );

        Self { config }
    }

    pub async fn run(&self) -> anyhow::Result<()> {
        init_telemetry()?;

        let cors_layer = CorsLayer::new()
            .allow_methods([Method::GET, Method::POST, Method::PUT])
            .allow_origin(HeaderValue::from_str(&self.config.request_host).unwrap())
            .allow_headers([AUTHORIZATION, CONTENT_TYPE])
            .allow_credentials(true);

        let governor_conf = GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(8)
            .finish()
            .unwrap();

        let routers = Router::new()
            .route("/register", post(register_user_handler))
            .route(
                "/confirm-email/{user_id}/{token}",
                get(confirm_email_handler),
            );

        let pem_content =
            std::fs::read(&self.config.private_key_path).expect("Failed to view EdDSA private key");

        let encoding_key = EncodingKey::from_ed_pem(&pem_content).expect("Invalid EdDSA key");

        let scylla_service = ScyllaService::new(
            self.config.hostnames.iter().map(String::as_str).collect(),
            &self.config.keyspace_name,
            &self.config.username,
            &self.config.password,
            self.config.case_sensitive,
        )
        .await?;

        let repository = ScyllaUserRepository::new(Arc::new(scylla_service));

        let state = AppState::new(Arc::new(repository), Arc::new(encoding_key));

        let app = Router::new()
            .nest("/auth", routers)
            .with_state(state)
            .layer(TraceLayer::new_for_http())
            .layer(GovernorLayer::new(governor_conf))
            .layer(cors_layer);

        let listener = tokio::net::TcpListener::bind(&self.config.addr)
            .await
            .unwrap();

        axum::serve(
            listener,
            app.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await?;

        return Ok(());
    }
}
