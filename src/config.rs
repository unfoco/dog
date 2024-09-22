use figment::{
    providers::{Env, Format, Json},
    Figment,
};
use poise::serenity_prelude as serenity;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Clone, Deserialize)]
pub struct WebhookChannel {
    pub webhook: String,
    pub channel: serenity::ChannelId,
}

impl WebhookChannel {
    pub async fn webhook<H>(&self, http: H) -> serenity::Result<serenity::Webhook>
    where
        H: AsRef<serenity::Http>,
    {
        serenity::Webhook::from_url(http, &self.webhook).await
    }
}

#[derive(Deserialize)]
pub struct LogChannels {
    pub system: serenity::ChannelId,
    pub member: serenity::ChannelId,
}

#[derive(Deserialize)]
pub struct Config {
    pub token: String,
    pub warns: Vec<serenity::RoleId>,
    pub boards: HashMap<String, WebhookChannel>,
    pub logs: LogChannels,
    pub admins: Vec<serenity::UserId>,
    pub autorole: serenity::RoleId,
    pub guild_id: serenity::GuildId,
}

impl Config {
    pub fn load() -> figment::error::Result<Self> {
        Figment::new()
            .merge(Json::file("Config.json"))
            .merge(Env::prefixed("DOG_"))
            .extract()
    }
}
