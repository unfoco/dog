use figment::{Figment, providers::{Format, Json, Env}};
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub token: String,
    pub prefix: String,
    pub boards: HashMap<String, String>,
    pub warns: Vec<u64>,
}

impl Config {
    pub fn load() -> figment::error::Result<Self> {
        Figment::new()
            .merge(Json::file("Config.json"))
            .merge(Env::prefixed("DOG_"))
            .extract()
    }
}
