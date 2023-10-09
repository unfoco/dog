use poise::serenity_prelude as serenity;
use serenity::model::prelude::Message;
use serenity::builder::CreateEmbed;

use crate::types;
use super::ExtendChannelId;

pub trait ExtendContext {
    async fn log_mem(&self, content: impl ToString) -> serenity::Result<Message>;
    async fn log_mem_with_embed(&self, content: impl ToString, f: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed) -> serenity::Result<Message>;
    async fn log_sys(&self, content: impl ToString) -> serenity::Result<Message>;
    async fn log_sys_with_embed(&self, content: impl ToString, f: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed) -> serenity::Result<Message>;

    async fn send_message(&self, content: impl ToString) -> serenity::Result<Message>;
}
impl ExtendContext for types::AppContext<'_> {
    async fn log_mem(&self, content: impl ToString) -> serenity::Result<Message> {
        self.data.log_mem(self.http(), content).await
    }

    async fn log_mem_with_embed(&self, content: impl ToString, f: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed) -> serenity::Result<Message> {
        self.data.log_mem_with_embed(self.http(), content, f).await
    }

    async fn log_sys(&self, content: impl ToString) -> serenity::Result<Message> {
        self.data.log_sys(self.http(), content).await
    }

    async fn log_sys_with_embed(&self, content: impl ToString, f: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed) -> serenity::Result<Message> {
        self.data.log_sys_with_embed(self.http(), content, f).await
    }

    async fn send_message(&self, content: impl ToString) -> serenity::Result<Message> {
        self.channel_id().send_message_content(self.http(), content).await
    }
}
