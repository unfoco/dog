use poise::serenity_prelude as serenity;
use serenity::model::prelude::Message;

use crate::types;

pub trait ExtendChannelId {
    async fn send_message_content(&self, http: impl AsRef<serenity::Http>, content: impl ToString) -> serenity::Result<Message>;
}

impl ExtendChannelId for serenity::ChannelId {
    async fn send_message_content(&self, http: impl AsRef<serenity::Http>, content: impl ToString) -> serenity::Result<Message> {
        self.send_message(http, |c| {
            c.content(content)
        })
    }
}
