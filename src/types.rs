use poise::serenity_prelude::*;

pub type Command = poise::Command<Data, Error>;

pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type ContextApp<'a> = poise::ApplicationContext<'a, Data, Error>;
pub type ContextFramework<'a> = poise::FrameworkContext<'a, Data, Error>;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type ErrorFramework<'a> = poise::FrameworkError<'a, Data, Error>;

pub type Framework = poise::Framework<Data, Error>;
pub type FrameworkOptions = poise::FrameworkOptions<Data, Error>;

pub struct Data {
    pub config: Config,
}

use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub guild: GuildId,
    pub admins: Vec<UserId>,
    pub boards: HashMap<String, Board>,
    pub roles: Roles,
    pub logs: Logs,
}

#[derive(Deserialize)]
pub struct Board {
    pub webhook: String,
    pub channel: ChannelId,
}

#[derive(Deserialize)]
pub struct Roles {
    pub default: RoleId,
    pub warnings: Vec<RoleId>,
}

#[derive(Deserialize)]
pub struct Logs {
    pub system: ChannelId,
    pub member: ChannelId,
}

impl Config {
    pub fn load() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::with_name("Config"))
            .add_source(config::Environment::with_prefix("DOG_"))
            .build()?
            .try_deserialize()
    }
}
