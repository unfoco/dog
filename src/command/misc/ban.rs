use poise::serenity_prelude as serenity;
use ::serenity::prelude::Mentionable;

use crate::types;

#[poise::command(prefix_command, slash_command, required_permissions = "BAN_MEMBERS")]
pub async fn ban(
    ctx: types::Context<'_>,
    member: serenity::Member,
    #[rest] reason: Option<String>,
) -> Result<(), types::Error> {

    let Some(guild) = ctx.guild_id() else {
        ctx.reply("this is a guild command").await?;
        return Ok(())
    };

    match reason {
        Some(reason) => {
            guild.ban_with_reason(ctx, member.user.id, 0, reason).await?;
        }
        None => guild.ban(ctx, member.user.id, 0).await?,
    }

    ctx.reply(format!("member {} banned", member.mention().to_string())).await?;
    Ok(())
}

#[poise::command(prefix_command, slash_command, required_permissions = "BAN_MEMBERS")]
pub async fn unban(
    ctx: types::Context<'_>,
    id: serenity::UserId,
) -> Result<(), types::Error> {

    let Some(guild) = ctx.guild_id() else {
        ctx.reply("this is a guild command").await?;
        return Ok(())
    };

    guild.unban(ctx, id).await?;

    ctx.reply(format!("member {} unbanned", id)).await?;
    Ok(())
}
