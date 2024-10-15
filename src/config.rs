use serde::{Deserialize, Serialize};
use config::{ConfigError, Config, File, FileFormat};


pub fn load_config() -> Result<Settings, ConfigError> {
    let settings = Config::builder()
        .add_source(File::new("config.yaml", FileFormat::Yaml))
        .build()?;
    settings.try_deserialize::<Settings>()
}


#[derive(Deserialize, Serialize)]
pub struct Settings {
    pub database: DatabaseSettings,
    pub application_port: u16,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseSettings {
    pub username: String,
    pub password: String,
    pub port: u16,
    pub host: String,
    pub database_name: String,
}


impl DatabaseSettings {
    pub fn connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.username,
            self.password,
            self.host,
            self.port,
            self.database_name,
        )
    }
}
