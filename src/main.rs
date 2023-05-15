use std::net::TcpListener;

use newsletters::{
    configuration::get_configuration,
    startup,
    telemetry::{get_subscriber, init_subscriber},
};
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("newsletters".into(), "info".into());
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Fail to read configuration");
    let connection_pool = PgPool::connect(&configuration.database.connection_string())
        .await
        .expect("Fail to connect to database");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listner = TcpListener::bind(address)?;
    startup::run(listner, connection_pool)?.await
}
