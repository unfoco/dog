use poise::serenity_prelude as serenity;
use chrono;

use crate::types;
use crate::util::macros::log_sys;

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

#[poise::command(context_menu_command = "mute", category = "admin", hide_in_help)]
pub async fn mute_user(
    ctx: types::AppContext<'_>,
    user: serenity::User,
) -> Result<(), types::Error> {
    mute(ctx, user).await
}

#[poise::command(context_menu_command = "user mute", category = "admin", hide_in_help)]
pub async fn mute_message(
    ctx: types::AppContext<'_>,
    msg: serenity::Message,
) -> Result<(), types::Error> {
    mute(ctx, msg.author).await
}

async fn mute(
    ctx: types::AppContext<'_>,
    user: serenity::User,
) -> Result<(), types::Error> {
    let Some(form) = ({
        poise::execute_modal(
            ctx,
            Some(MuteModal {
                reason: format!("@{} adlı üyenin mute sebebi", user.name),
                duration: "".to_string(),
            }),
            None
        ).await?
    }) else {
        return Ok(())
    };

    let guild = ctx.guild_id().unwrap();
    let mut member = guild.member(ctx.http(), &user.id).await?;

    member.disable_communication_until_datetime(
        ctx.http(),
        serenity::Timestamp::from(
            chrono::Utc::now() + duration_str::parse(&form.duration).unwrap()
        ),
    ).await?;

    ctx.channel_id().send_message(ctx.http(), |c| {
        c.content(format!("{} adlı üye {} süreliğine mutelendi", user, &form.duration))
    }).await?;

    log_sys!("{} adlı üye {} süreliğine {} tarafından mutelendi", user, &form.duration, ctx.author());

    return Ok(())
}
