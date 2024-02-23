use ::serenity::prelude::Mentionable;
use poise::serenity_prelude as serenity;

use crate::types;

pub async fn handle(
    ctx: &serenity::Context,
    _framework: types::FrameworkContext<'_>,
    data: &types::Data,
    channel_id: &serenity::ChannelId,
    message_id: &serenity::MessageId,
    _guild_id: &Option<serenity::GuildId>,
) -> Result<(), types::Error> {
    let message = ctx.cache.message(channel_id, message_id);

    let Some(message) = message else {
        return Ok(());
    };

    if message.guild_id.is_none() {
        return Ok(());
    }

    let logs = &data.config.logs;

    if *channel_id == logs.member || *channel_id == logs.system {
        return Ok(());
    }

    let log = logs.member;

    log.send_message(ctx, |c| {
        c.add_embed(|c| {
            c.description(format!(
                "{} kanalında {} tarafından gönderilen bir mesaj kaldırıldı",
                channel_id.mention(),
                message.author,
            ))
        });

        if !message.content.is_empty() {
            c.add_embed(|c| c.description(message.content));
        }

        for attachment in &message.attachments {
            c.add_file(serenity::AttachmentType::Image(
                url::Url::parse(&attachment.url).unwrap(),
            ));
        }

        c
    })
    .await?;

    Ok(())
}
