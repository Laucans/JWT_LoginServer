use super::config_structs::AppConfig;
use config::{builder::AsyncState, Config, ConfigBuilder, File};
use serde::de::DeserializeOwned;
use std::fs;

trait ConfigExt {
    fn get_option<T: DeserializeOwned>(&self, key: &str) -> Option<T>;
}

impl ConfigExt for Config {
    fn get_option<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        match self.get::<T>(key) {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    }
}

pub async fn read_from_disk<T: DeserializeOwned>(key: &str) -> Option<T> {
    let config = ConfigBuilder::<AsyncState>::default()
        .add_source(File::with_name("config/Config.toml"))
        .build()
        .await
        .unwrap();
    config.get_option::<T>(key)
}

pub async fn load_config() -> AppConfig {
    let config_str = fs::read_to_string("config/Config.toml").unwrap();
    toml::from_str(&config_str).unwrap()
}
