use poise::serenity_prelude as serenity;
use ::serenity::model::prelude::Message;
use ::serenity::builder::CreateEmbed;
use async_trait::async_trait;

use crate::types;
use super::ExtendChannelId;

#[async_trait]
pub trait ExtendContext {
    async fn log_mem(&self, content: impl ToString + Send) -> serenity::Result<Message>;
    async fn log_mem_with_embed<F>(&self, content: impl ToString + Send, f: F) -> serenity::Result<Message>
        where F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send + Sync;
    async fn log_sys(&self, content: impl ToString + Send) -> serenity::Result<Message>;
    async fn log_sys_with_embed<F>(&self, content: impl ToString + Send, f: F) -> serenity::Result<Message>
        where F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send + Sync;

    async fn send_message(&self, content: impl ToString + Send) -> serenity::Result<Message>;
}

#[async_trait]
impl ExtendContext for types::AppContext<'_> {
    async fn log_mem(&self, content: impl ToString + Send) -> serenity::Result<Message> {
        self.data.log_mem(self.http(), content).await
    }

    async fn log_mem_with_embed<F>(&self, content: impl ToString + Send, f: F) -> serenity::Result<Message>
        where F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send + Sync
    {
        self.data.log_mem_with_embed(self.http(), content, f).await
    }

    async fn log_sys(&self, content: impl ToString + Send) -> serenity::Result<Message> {
        self.data.log_sys(self.http(), content).await
    }

    async fn log_sys_with_embed<F>(&self, content: impl ToString + Send, f: F) -> serenity::Result<Message>
        where F: FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send + Sync
    {
        self.data.log_sys_with_embed(self.http(), content, f).await
    }

    async fn send_message(&self, content: impl ToString + Send) -> serenity::Result<Message> {
        self.channel_id().send_message_content(self.http(), content).await
    }
}
