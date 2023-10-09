use poise::serenity_prelude as serenity;
use ::serenity::prelude::Mentionable;

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
        return Ok(())
    };

    if message.guild_id.is_none() {
        return Ok(())
    }

    if message.author.bot {
        return Ok(())
    }

    let log = data.config.logs.member;

    log.send_message(ctx, |c| {
        c.content(format!(
            "{} kanalında {} tarafından gönderilen bir mesaj silindi",
            channel_id.mention(),
            message.author,
        ));

        for attachment in &message.attachments {
            c.add_file(serenity::AttachmentType::Image(
                url::Url::parse(&attachment.url).unwrap()
            ));
        }

        if !message.content.is_empty() {
            c.embed(|c| {
                c.description(message.content)
            });
        }
        c
    }).await?;

    Ok(())
}
