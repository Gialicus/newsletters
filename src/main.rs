use std::net::TcpListener;

use newsletters::{configuration::get_configuration, startup};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let configuration = get_configuration().expect("Fail to read configuration");
    let address = format!("127.0.0.1:{}", configuration.application_port);
    let listner = TcpListener::bind(address)?;
    startup::run(listner)?.await
}
