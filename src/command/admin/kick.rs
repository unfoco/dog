use poise::serenity_prelude as serenity;

use crate::types;

#[derive(Debug, poise::Modal)]
#[name = "kick"]
#[allow(dead_code)]
struct KickModal {
    #[paragraph]
    #[max_length = 1024]
    #[name = "sebebi"]
    reason: String,
}

#[poise::command(context_menu_command = "kick", category = "admin", hide_in_help)]
pub async fn kick_user(
    ctx: types::AppContext<'_>,
    user: serenity::User,
) -> Result<(), types::Error> {
    kick(ctx, user).await
}

#[poise::command(context_menu_command = "user kick", category = "admin", hide_in_help)]
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
    let Some(form) = ({
        poise::execute_modal(
            ctx,
            Some(KickModal{
                reason: format!("@{} adlı üyenin kick sebebi", user.name)
            }),
            None
        ).await?
    }) else {
        return Ok(())
    };

    let guild = ctx.guild_id().unwrap();

    guild.kick_with_reason(ctx.http(), user.id, &form.reason).await?;

    ctx.channel_id().send_message(ctx.http(), |c| {
        c.content(format!("{} adlı üye kicklendi", user))
    }).await?;

    ctx.data.log_sys(
        ctx.http(),
        format!("{} adlı üye {} tarafından kicklendi", user, ctx.author())
    ).await?;

    return Ok(())
}
