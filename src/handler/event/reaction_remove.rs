use poise::serenity_prelude as serenity;

use crate::types;

pub async fn handle(
    ctx: &serenity::Context,
    _framework: types::FrameworkContext<'_>,
    data: &types::Data,
    reaction: &serenity::Reaction
) -> Result<(), types::Error> {
    let Some(member) = &reaction.member else {
        return Ok(())
    };

    let Some(user) = &member.user else {
        return Ok(())
    };

    data.log_mem(ctx, format!(
        "{} {} mesajından {} tepkisini kaldırdı",
        user,
        reaction.message_id.link(reaction.channel_id, reaction.guild_id),
        reaction.emoji,
    )).await?;

    Ok(())
}
