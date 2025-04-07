use poise::serenity_prelude as serenity;
use serenity::Mentionable;

use crate::types;

pub async fn handle(
    ctx: &serenity::Context,
    _framework: types::ContextFramework<'_>,
    data: &types::Data,
    channel_id: &serenity::ChannelId,
    message_id: &serenity::MessageId,
    guild_id: &Option<serenity::GuildId>,
) -> Result<(), types::Error> {
    let message = ctx.cache
        .message(channel_id, message_id)
        .map(|x| x.to_owned());

    let Some(message) = message else {
        return Ok(());
    };

    if guild_id.is_none() {
        return Ok(());
    }

    let logs = &data.config.logs;

    if *channel_id == logs.member || *channel_id == logs.system {
        return Ok(());
    }

    let mut builder = serenity::CreateMessage::new()
        .add_embed(
            serenity::CreateEmbed::new()
                .description(format!(
                    "{} kanalında {} tarafından gönderilen bir mesaj kaldırıldı",
                    channel_id.mention(),
                    message.author,
                ))
        );

    if !message.content.is_empty() {
        builder = builder.add_embed(
            serenity::CreateEmbed::new()
                .description(&message.content)
        );
    }

    for attachment in &message.attachments {
        let file = serenity::CreateAttachment::url(ctx, &attachment.url)
            .await?;

        builder = builder.add_file(file);
    }

    logs.member.send_message(ctx, builder)
        .await?;

    Ok(())
}
