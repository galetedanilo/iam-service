use crate::presentation::api::Service;

mod application;
mod domain;
mod infrastructure;
mod presentation;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();

    Service::start().run().await
}
