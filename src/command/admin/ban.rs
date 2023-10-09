use poise::serenity_prelude as serenity;

use crate::types;

use crate::util;
use crate::util::macros::log_sys;
use crate::util::traits::ExtendContext;

#[derive(Debug, poise::Modal)]
#[name = "ban"]
#[allow(dead_code)]
struct BanModal {
    #[paragraph]
    #[max_length = 1024]
    #[name = "sebebi"]
    reason: String,
}

#[poise::command(context_menu_command = "ban", category = "admin", guild_only, hide_in_help)]
pub async fn ban_user(
    ctx: types::AppContext<'_>,
    user: serenity::User,
) -> Result<(), types::Error> {
    ban(ctx, user).await
}

#[poise::command(context_menu_command = "user ban", category = "admin", guild_only, hide_in_help)]
pub async fn ban_message(
    ctx: types::AppContext<'_>,
    msg: serenity::Message,
) -> Result<(), types::Error> {
    ban(ctx, msg.author).await
}

async fn ban(
    ctx: types::AppContext<'_>,
    user: serenity::User,
) -> Result<(), types::Error> {
    let guild = ctx.guild().unwrap();

    let Ok(bans) = guild.bans(ctx.http()).await else {
        return Ok(())
    };

    if bans.iter().find_map(|b| {
        if b.user == user {
            Some(())
        } else {
            None
        }
    }).is_some() {
        return unban(ctx, user, guild).await
    }

    let Some(form) = ({
        poise::execute_modal(
            ctx,
            Some(BanModal {
                reason: format!("@{} adlı üyenin ban sebebi", user.name),
            }),
            None
        ).await?
    }) else {
        return Ok(())
    };

    let member = guild.member(ctx.http(), &user.id).await?;

    if let Err(_) = member.ban_with_reason(
        ctx.http(), 0,
        &form.reason,
    ).await {
        ctx.send(|c| {
            c.content("üye bulunamadığından veya yetkisi yüksek olduğundan banlanamadı");
            c.ephemeral(true)
        }).await?;
        return Ok(())
    }

    ctx.send_message(format!("{} adlı üye banlandı", user)).await?;

    log_sys!(ctx, "{} adlı üye {} tarafından banlandı", user, ctx.author());

    return Ok(())
}

async fn unban(
    ctx: types::AppContext<'_>,
    user: serenity::User,
    guild: serenity::Guild,
) -> Result<(), types::Error> {
    let result = util::send_confirmation(
        ctx, "bu üye zaten banlı banı kaldırmak istiyor musunuz?"
    ).await?;

    if result {
        guild.unban(ctx.http(), &user).await?;

        ctx.send_message(format!("{} adlı üyenin banı kaldırıldı", user)).await?;

        log_sys!(ctx, "{} adlı üyenin banı {} tarafından kaldırıldı", user, ctx.author());
    }

    Ok(())
}
