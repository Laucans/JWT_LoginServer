use crate::deserializer;
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct AppConfig {
    pub server: Server,
    pub administration: Administration,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Server {
    pub address: String,
    pub port: u16,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Administration {
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "deserializer::custom::base64_decode")]
    pub secret_key: Vec<u8>,
    #[serde(default = "default_logger_scope")]
    pub logger_scope: String,
}

/* DEFAULT VALUE SECTION */
fn default_logger_scope() -> String {
    "info".to_string()
}
