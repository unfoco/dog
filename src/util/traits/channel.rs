use ::serenity::model::prelude::Message;
use async_trait::async_trait;
use poise::serenity_prelude as serenity;

#[async_trait]
pub trait ExtendChannelId {
    async fn send_message_content<H, C>(&self, http: H, content: C) -> serenity::Result<Message>
    where
        H: AsRef<serenity::Http> + Send + Sync,
        C: ToString + Send;
}

#[async_trait]
impl ExtendChannelId for serenity::ChannelId {
    async fn send_message_content<H, C>(&self, http: H, content: C) -> serenity::Result<Message>
    where
        H: AsRef<serenity::Http> + Send + Sync,
        C: ToString + Send,
    {
        self.send_message(http, |c| c.content(content)).await
    }
}
