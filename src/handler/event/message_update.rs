use poise::serenity_prelude as serenity;
use ::serenity::prelude::Mentionable;

use crate::types;

pub async fn handle(
    ctx: &serenity::Context,
    _framework: types::FrameworkContext<'_>,
    data: &types::Data,
    old: &Option<serenity::Message>,
    new: &Option<serenity::Message>,
    _event: &serenity::MessageUpdateEvent,
) -> Result<(), types::Error> {
    let Some(old_message) = old else {
        return Ok(())
    };

    let Some(new_message) = new else {
        return Ok(())
    };

    if old_message.author.bot || old_message.content == new_message.content {
        return Ok(())
    }

    data.log_mem_with_embed(
        ctx,
        format!(
            "{} {} kanalında gönderdiği bir mesajı düzenledi",
            old_message.author,
            old_message.channel_id.mention(),
        ),
        |c| {
            c.field("eski", old_message.content.clone(), true);
            c.field("yeni", new_message.content.clone(), true)
        },
    ).await?;

    Ok(())
}
