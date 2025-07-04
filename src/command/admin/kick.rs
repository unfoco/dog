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

#[poise::command(
    context_menu_command = "kick",
    category = "admin",
    guild_only,
    hide_in_help
)]
pub async fn kick_user(
    ctx: types::ContextApp<'_>,
    user: serenity::User,
) -> Result<(), types::Error> {
    kick(ctx, user).await
}

#[poise::command(
    context_menu_command = "user kick",
    category = "admin",
    guild_only,
    hide_in_help
)]
pub async fn kick_message(
    ctx: types::ContextApp<'_>,
    msg: serenity::Message,
) -> Result<(), types::Error> {
    kick(ctx, msg.author).await
}

async fn kick(ctx: types::ContextApp<'_>, user: serenity::User) -> Result<(), types::Error> {
    let guild = ctx.guild_id().unwrap();

    if guild.member(ctx, &user.id).await.is_err() {
        ctx.send(
            poise::CreateReply::default()
                .content("üye bulunamadığından atılamadı")
                .ephemeral(true)
        ).await?;
        return Ok(());
    };

    let Some(form) = ({
        poise::execute_modal(
            ctx,
            Some(KickModal {
                reason: format!("@{} atılma sebebi", user.name),
            }),
            None,
        )
        .await?
    }) else {
        return Ok(());
    };

    guild
        .kick_with_reason(ctx, user.id, &form.reason)
        .await?;

    //ctx.send_message(format!("{} atıldı", user)).await?;

    //log_sys!(ctx, "{} {} tarafından atıldı", user, ctx.author());

    return Ok(());
}
