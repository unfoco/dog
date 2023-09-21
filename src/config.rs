use figment::{Figment, providers::{Format, Json, Env}};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub token: String,
    pub prefix: String,
    pub boards: Vec<Board>,
}

#[derive(Debug, Deserialize)]
pub struct Board {
    pub name: String,
    pub room: String,
    pub webhook: String,
}

impl Config {
    pub fn load() -> figment::error::Result<Self> {
        Figment::new()
            .merge(Json::file("Config.json"))
            .merge(Env::prefixed("DOG_"))
            .extract()
    }
}
