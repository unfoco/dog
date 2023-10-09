use poise::serenity_prelude as serenity;
use serenity::model::prelude::Message;
use async_trait::async_trait;

#[async_trait]
pub trait ExtendChannelId {
    async fn send_message_content(&self, http: impl AsRef<serenity::Http> + Send + Sync, content: impl ToString + Send) -> serenity::Result<Message>;
}

#[async_trait]
impl ExtendChannelId for serenity::ChannelId {
    async fn send_message_content(&self, http: impl AsRef<serenity::Http> + Send + Sync, content: impl ToString + Send) -> serenity::Result<Message> {
        self.send_message(http, |c| {
            c.content(content)
        }).await
    }
}
