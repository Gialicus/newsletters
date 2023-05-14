use std::net::TcpListener;

use newsletters::{configuration::get_configuration, startup};
use sqlx::PgPool;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{layer::SubscriberExt, EnvFilter, Registry};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    let formatting_layer = BunyanFormattingLayer::new("newsletters".into(), std::io::stdout);
    let subscriber = Registry::default()
        .with(env_filter)
        .with(JsonStorageLayer)
        .with(formatting_layer);
    set_global_default(subscriber).expect("Fail to set subscriber");
    let configuration = get_configuration().expect("Fail to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Fail to connect to database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listner = TcpListener::bind(address)?;
    startup::run(listner, connection_pool)?.await
}
