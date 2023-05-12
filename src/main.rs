use std::net::TcpListener;

use newsletters::run;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listner = TcpListener::bind("127.0.0.1:8000").expect("Failed to bind address");
    run(listner)?.await
}
