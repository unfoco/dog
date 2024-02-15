use poise::serenity_prelude as serenity;

use crate::types;
use crate::util::macros::log_sys;
use crate::util::traits::ExtendContext;

#[derive(Debug, poise::Modal)]
#[name = "kick"]
#[allow(dead_code)]
struct KickModal {
    #[paragraph]
    #[max_length = 1024]
    #[name = "sebebi"]
    reason: String,
}

#[poise::command(context_menu_command = "kick", category = "admin", guild_only, hide_in_help)]
pub async fn kick_user(
    ctx: types::AppContext<'_>,
    user: serenity::User,
) -> Result<(), types::Error> {
    kick(ctx, user).await
}

#[poise::command(context_menu_command = "user kick", category = "admin", guild_only, hide_in_help)]
pub async fn kick_message(
    ctx: types::AppContext<'_>,
    msg: serenity::Message,
) -> Result<(), types::Error> {
    kick(ctx, msg.author).await
}

async fn kick(
    ctx: types::AppContext<'_>,
    user: serenity::User,
) -> Result<(), types::Error> {
    if user.member.is_none() {
        ctx.send(|c| {
            c.content("üye bulunamadığından atılamadı");
            c.ephemeral(true)
        }).await?;
        return Ok(())
    }

    let Some(form) = ({
        poise::execute_modal(
            ctx,
            Some(KickModal{
                reason: format!("@{} atılma sebebi", user.name)
            }),
            None
        ).await?
    }) else {
        return Ok(())
    };

    let guild = ctx.guild_id().unwrap();

    guild.kick_with_reason(ctx.http(), user.id, &form.reason).await?;

    ctx.send_message(format!("{} atıldı", user)).await?;

    log_sys!(ctx, "{} {} tarafından atıldı", user, ctx.author());

    return Ok(())
}
