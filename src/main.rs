use tokio;
use std::net::TcpListener;
use sqlx::PgPool;

use antenna::startup::run;
use antenna::config::load_config;


#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    let config = load_config().expect("Failed to load configuration");
    let connection = PgPool::connect(&config.database.connection_string())
            .await
            .expect("Failed to connect to the database");
    let address = format!("{}:{}", config.database.host, config.application_port);
    let tcp = TcpListener::bind(address).unwrap();
    let server = run(tcp, connection).unwrap();
    let _ = server.await;
    Ok(())
}

