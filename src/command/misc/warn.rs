use poise::serenity_prelude as serenity;

use crate::types;

#[poise::command(prefix_command, slash_command, required_permissions = "BAN_MEMBERS")]
pub async fn warn(
    ctx: types::Context<'_>,
    member: serenity::Member,
) -> Result<(), types::Error> {

    let Some(guild) = ctx.guild_id() else {
        ctx.reply("this is a guild command").await?;
        return Ok(())
    };
    Ok(())
}