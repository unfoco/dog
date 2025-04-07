use poise::serenity_prelude as serenity;

use crate::types;

pub async fn handle(
    ctx: &serenity::Context,
    _framework: types::ContextFramework<'_>,
    data: &types::Data,
    reaction: &serenity::Reaction,
) -> Result<(), types::Error> {
    let Some(member) = &reaction.member else {
        return Ok(());
    };

    let embed = serenity::CreateEmbed::new()
    .description(format!(
        "{} {} mesajından {} tepkisini kaldırdı",
        member.user,
        reaction
            .message_id
            .link(reaction.channel_id, reaction.guild_id),
        reaction.emoji,
    ));

    let logs = &data.config.logs;

    logs.member.send_message(
        ctx, serenity::CreateMessage::new()
            .add_embed(embed)
    ).await?;

    Ok(())
}
