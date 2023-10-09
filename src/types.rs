use poise::serenity_prelude as serenity;
use ::serenity::builder::CreateEmbed;

use crate::config::Config;

pub type CommandVec = Vec<poise::Command<Data, Error>>;

pub type Framework = poise::Framework<Data, Error>;
pub type FrameworkOptions = poise::FrameworkOptions<Data, Error>;

pub type Context<'a> = poise::Context<'a, Data, Error>;
pub type AppContext<'a> = poise::ApplicationContext<'a, Data, Error>;
pub type FrameworkContext<'a> = poise::FrameworkContext<'a, Data, Error>;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type FrameworkError<'a> = poise::FrameworkError<'a, Data, Error>;

pub struct Data {
    pub config: Config
}

impl Data {

    pub async fn log_mem(
        &self,
        http: impl AsRef<serenity::Http>,
        content: impl ToString
    ) -> serenity::Result<serenity::model::prelude::Message> {
        self.config.logs.member.send_message(http, |c| {
            c.content(content)
        }).await
    }

    pub async fn log_mem_with_embed(
        &self,
        http: impl AsRef<serenity::Http>,
        content: impl ToString,
        f: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
    ) -> serenity::Result<serenity::model::prelude::Message> {
        self.config.logs.member.send_message(http, |c| {
            c.content(content);
            c.embed(f)
        }).await
    }

    pub async fn log_sys(
        &self,
        http: impl AsRef<serenity::Http>,
        content: impl ToString
    ) -> serenity::Result<serenity::model::prelude::Message> {
        self.config.logs.system.send_message(http, |c| {
            c.content(content)
        }).await
    }

    pub async fn log_sys_with_embed(
        &self,
        http: impl AsRef<serenity::Http>,
        content: impl ToString,
        f: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
    ) -> serenity::Result<serenity::model::prelude::Message> {
        self.config.logs.system.send_message(http, |c| {
            c.content(content);
            c.embed(f)
        }).await
    }
}
