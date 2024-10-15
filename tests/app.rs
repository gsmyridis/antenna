use std::net::TcpListener;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

use antenna::startup;
use antenna::config::{load_config, DatabaseSettings};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}


// Spawn app starts our application and its implementation will
// depend on the framework. 
//
// Whith this abstraction the rest of the test-code is framework-independent.
// In fact, even if tomorrow we decided to implement our application in another 
// language altogether, the test-code would still be valid.
pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0")
        .expect("Failed to bind to random port");
    let address = listener.local_addr()
        .expect("Failed to get local address");

    let mut config = load_config().expect("Failed to load configuration");
    config.database.database_name = Uuid::new_v4().to_string();
    let connection_pool = configure_database(&config.database).await;

    let server = startup::run(listener, connection_pool.clone()).expect("Failed to spawn app");
    let _ = tokio::spawn(server);
    TestApp { address: format!("http://{}", address.to_string()), db_pool: connection_pool }
}


/// Uses the "maintainance" postgress database that comes with the installation
/// to spin up a new database for the test runs.
pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    
    let maintainance_settings = DatabaseSettings {
        database_name: "postgres".to_string(),
        username: "postgres".to_string(),
        password: "password".to_string(),
        ..config.clone()
    };

    // Create new database
    let mut connection = PgConnection::connect(
        &maintainance_settings.connection_string()
    ).await
     .expect("Failed to connect to Postgres");

    connection.execute(
        format!(r#"CREATE DATABASE "{}";"#, config.database_name).as_str()
    ).await
     .expect("Failed to create database.");

    // Migrate database
    let connection_pool = PgPool::connect(&config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&connection_pool)
        .await
        .expect("Failed to migrate the database");

    connection_pool
}
