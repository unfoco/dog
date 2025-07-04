use poise::serenity_prelude as serenity;
use serenity::Mentionable;

use crate::types;

pub async fn handle(
    ctx: &serenity::Context,
    _framework: types::ContextFramework<'_>,
    data: &types::Data,
    old: &Option<serenity::Message>,
    new: &Option<serenity::Message>,
    _event: &serenity::MessageUpdateEvent,
) -> Result<(), types::Error> {
    let Some(old_message) = old else {
        return Ok(());
    };

    let Some(new_message) = new else {
        return Ok(());
    };

    if old_message.guild_id.is_none()
        || old_message.author.bot
        || old_message.content == new_message.content
    {
        return Ok(());
    }

    let embed = serenity::CreateEmbed::new()
        .description(format!(
            "{} {} kanalında gönderdiği bir mesajı düzenledi",
            old_message.author,
            old_message.channel_id.mention(),
        ))
        .field("eski", old_message.content.clone(), true)
        .field("yeni", new_message.content.clone(), true);

    let logs = &data.config.logs;

    logs.member.send_message(
        ctx, serenity::CreateMessage::new()
            .add_embed(embed)
    ).await?;

    Ok(())
}
