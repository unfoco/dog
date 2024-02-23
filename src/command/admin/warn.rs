use poise::serenity_prelude as serenity;

use crate::types;
use crate::util::macros::log_sys;
use crate::util::traits::ExtendContext;

#[derive(Debug, poise::Modal)]
#[name = "warn"]
#[allow(dead_code)]
struct WarnModal {
    #[paragraph]
    #[max_length = 1024]
    #[name = "sebebi"]
    reason: String,
}

#[poise::command(
    context_menu_command = "warn",
    category = "admin",
    guild_only,
    hide_in_help
)]
pub async fn warn_user(
    ctx: types::AppContext<'_>,
    user: serenity::User,
) -> Result<(), types::Error> {
    warn(ctx, user).await
}

#[poise::command(
    context_menu_command = "user warn",
    category = "admin",
    guild_only,
    hide_in_help
)]
pub async fn warn_message(
    ctx: types::AppContext<'_>,
    msg: serenity::Message,
) -> Result<(), types::Error> {
    warn(ctx, msg.author).await
}

async fn warn(ctx: types::AppContext<'_>, user: serenity::User) -> Result<(), types::Error> {
    let guild = ctx.guild_id().unwrap();

    let Ok(mut member) = guild.member(ctx.http(), &user.id).await else {
        ctx.send(|c| {
            c.content("üye bulunamadığından uyarılamadı");
            c.ephemeral(true)
        })
        .await?;
        return Ok(());
    };

    let Some(form) = ({
        poise::execute_modal(
            ctx,
            Some(WarnModal {
                reason: format!("@{} uyarı sebebi", user.name),
            }),
            None,
        )
        .await?
    }) else {
        return Ok(());
    };

    let warns = &ctx.data.config.warns;

    let Some(role) = warns.iter().find_map(|role| {
        if !member.roles.contains(role) {
            Some(role)
        } else {
            None
        }
    }) else {
        ctx.send(|c| {
            c.content("üye zaten yeterince uyarı aldı");
            c.ephemeral(true)
        })
        .await?;
        return Ok(());
    };

    member.add_role(ctx.http(), role).await?;

    ctx.send_message(format!("{} uyarıldı", user)).await?;

    ctx.log_sys_with_embed(
        format!("{} {} tarafından uyarıldı", user, ctx.author()),
        |c| c.field("sebep", form.reason, true),
    )
    .await?;

    if warns.iter().all(|r| member.roles.contains(r)) {
        ctx.send_message(
            "üye uyarı hakkını doldurduğundan yönetim cezaya karar veresiye kadar susturulmuştur",
        )
        .await?;

        if let Some(time) = member.communication_disabled_until {
            log_sys!(
                ctx,
                "{} eski susturmasının bitmesine <t:{}:R>",
                user,
                time.timestamp()
            );
        }

        member
            .disable_communication_until_datetime(
                ctx.http(),
                serenity::Timestamp::from(chrono::Utc::now() + duration_str::parse("24d").unwrap()),
            )
            .await?;

        ctx.send_message(format!(
            "{} cezasına karar verilesiye kadar susturuldu",
            user
        ))
        .await?;
    }

    Ok(())
}

#[poise::command(
    context_menu_command = "unwarn",
    category = "admin",
    guild_only,
    hide_in_help
)]
pub async fn unwarn_user(
    ctx: types::AppContext<'_>,
    user: serenity::User,
) -> Result<(), types::Error> {
    unwarn(ctx, user).await
}

#[poise::command(
    context_menu_command = "user unwarn",
    category = "admin",
    guild_only,
    hide_in_help
)]
pub async fn unwarn_message(
    ctx: types::AppContext<'_>,
    msg: serenity::Message,
) -> Result<(), types::Error> {
    unwarn(ctx, msg.author).await
}

pub async fn unwarn(ctx: types::AppContext<'_>, user: serenity::User) -> Result<(), types::Error> {
    let guild = ctx.guild_id().unwrap();

    let Ok(mut member) = guild.member(ctx.http(), &user.id).await else {
        ctx.send(|c| {
            c.content("üye bulunamadığından uyarı kaldırılamadı");
            c.ephemeral(true)
        })
        .await?;
        return Ok(());
    };

    let warns = ctx.data.config.warns.clone();

    let Some(role) = warns.iter().rev().find_map(|role| {
        if member.roles.contains(role) {
            Some(role)
        } else {
            None
        }
    }) else {
        ctx.send(|c| {
            c.content("üye hiç uyarı almamış");
            c.ephemeral(true)
        })
        .await?;
        return Ok(());
    };

    member.remove_role(ctx.http(), role).await?;

    ctx.send_message(format!("{} bir uyarısı kaldırıldı", member))
        .await?;

    log_sys!(
        ctx,
        "{} bir uyarısı {} tarafından kaldırıldı",
        member,
        ctx.author()
    );

    Ok(())
}
