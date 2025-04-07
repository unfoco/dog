use chrono;
use poise::serenity_prelude as serenity;

use crate::types;

#[derive(Debug, poise::Modal)]
#[name = "mute"]
#[allow(dead_code)]
struct MuteModal {
    #[paragraph]
    #[max_length = 1024]
    #[name = "sebebi"]
    reason: String,
    #[name = "süre (max: 27d)"]
    duration: String,
}

#[poise::command(
    context_menu_command = "mute",
    category = "admin",
    guild_only,
    hide_in_help
)]
pub async fn mute_user(
    ctx: types::ContextApp<'_>,
    user: serenity::User,
) -> Result<(), types::Error> {
    mute(ctx, user).await
}

#[poise::command(
    context_menu_command = "user mute",
    category = "admin",
    guild_only,
    hide_in_help
)]
pub async fn mute_message(
    ctx: types::ContextApp<'_>,
    msg: serenity::Message,
) -> Result<(), types::Error> {
    mute(ctx, msg.author).await
}

async fn mute(ctx: types::ContextApp<'_>, user: serenity::User) -> Result<(), types::Error> {
    let guild = ctx.guild_id().unwrap();

    let Ok(mut member) = guild.member(ctx.http(), &user.id).await else {
        ctx.send(|c| {
            c.content("üye bulunamadığından susturulamadı");
            c.ephemeral(true)
        })
        .await?;
        return Ok(());
    };

    if member.communication_disabled_until.is_some() {
        return unmute(ctx, user, guild, member).await;
    }

    let Some(form) = ({
        poise::execute_modal(
            ctx,
            Some(MuteModal {
                reason: format!("@{} susturma sebebi", user.name),
                duration: "".to_string(),
            }),
            None,
        )
        .await?
    }) else {
        return Ok(());
    };

    if let Err(_) = member
        .disable_communication_until_datetime(
            ctx.http(),
            serenity::Timestamp::from(
                chrono::Utc::now() + duration_str::parse(&form.duration).unwrap(),
            ),
        )
        .await
    {
        ctx.send(|c| {
            c.content("üye susturulmadı");
            c.ephemeral(true)
        })
        .await?;
        return Ok(());
    }

    ctx.send_message(format!("{} {} süreliğine susturuldu", user, &form.duration))
        .await?;

    log_sys!(
        ctx,
        "{} {} süreliğine {} tarafından susturuldu",
        user,
        &form.duration,
        ctx.author()
    );

    return Ok(());
}

async fn unmute(
    ctx: types::ContextApp<'_>,
    user: serenity::User,
    _guild: serenity::GuildId,
    mut member: serenity::Member,
) -> Result<(), types::Error> {
    let result = util::interactions::send_confirm(
        ctx,
        "bu üye zaten susturulmuş susturmayı kaldırmak istiyor musunuz?",
    )
    .await?;

    if result {
        member.enable_communication(ctx.http()).await?;

        ctx.send_message(format!("{} susturması kaldırıldı", user))
            .await?;

        log_sys!(
            ctx,
            "{} susturması {} tarafından kaldırıldı",
            user,
            ctx.author()
        );
    }

    Ok(())
}
