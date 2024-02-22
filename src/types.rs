use ::serenity::builder::CreateEmbed;
use ::serenity::model::prelude::Message;
use poise::serenity_prelude as serenity;

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
    pub config: Config,
}

impl Data {
    pub async fn log_mem<H, C>(&self, http: H, content: C) -> serenity::Result<Message>
    where
        H: AsRef<serenity::Http>,
        C: ToString,
    {
        self.config
            .logs
            .member
            .send_message(http, |c| c.embed(|c| c.description(content)))
            .await
    }

    pub async fn log_mem_with_embed<H, C, F>(
        &self,
        http: H,
        content: C,
        f: F,
    ) -> serenity::Result<Message>
    where
        H: AsRef<serenity::Http>,
        C: ToString,
        F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
    {
        self.config
            .logs
            .member
            .send_message(http, |c| {
                c.embed(|c| {
                    let n = f(c);
                    n.description(content);
                    n
                })
            })
            .await
    }

    pub async fn log_sys<H, C>(&self, http: H, content: C) -> serenity::Result<Message>
    where
        H: AsRef<serenity::Http>,
        C: ToString,
    {
        self.config
            .logs
            .system
            .send_message(http, |c| c.embed(|c| c.description(content)))
            .await
    }

    pub async fn log_sys_with_embed<H, C, F>(
        &self,
        http: H,
        content: C,
        f: F,
    ) -> serenity::Result<Message>
    where
        H: AsRef<serenity::Http>,
        C: ToString,
        F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
    {
        self.config
            .logs
            .system
            .send_message(http, |c| {
                c.embed(|c| {
                    let n = f(c);
                    n.description(content);
                    n
                })
            })
            .await
    }
}
