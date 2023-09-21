use poise::serenity_prelude as serenity;
use serenity::prelude::Mentionable;

use crate::types;

#[poise::command(prefix_command, slash_command, required_permissions = "BAN_MEMBERS")]
pub async fn warn(
    ctx: types::Context<'_>,
    mut member: serenity::Member,
) -> Result<(), types::Error> {

    let Some(guild) = ctx.guild_id() else {
        ctx.reply("this is a guild command").await?;
        return Ok(())
    };

    let contains = |id: u64| {
        member.roles.contains(
            &serenity::RoleId(id)
        )
    };

    let warn: Option<serenity::RoleId> = ctx.data()
        .config.warns.iter()
        .filter_map(|w| {
            if !contains(*w) {
                Some(serenity::RoleId(*w))
            } else {
                None
            }
        }).next();

    if let Some(role) = warn {
        member.add_roles(ctx, &[role]).await?;
        ctx.reply(format!("{} warned", member.mention())).await?;
    } else {
        guild.ban_with_reason(ctx, member.user.id, 0, "warn").await?;
        ctx.reply(format!("{} banned", member.mention())).await?;
    }

    Ok(())
}
