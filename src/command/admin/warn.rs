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

#[poise::command(context_menu_command = "warn", category = "admin", hide_in_help)]
pub async fn warn_user(
    ctx: types::AppContext<'_>,
    user: serenity::User,
) -> Result<(), types::Error> {
    warn(ctx, user).await
}

#[poise::command(context_menu_command = "user warn", category = "admin", hide_in_help)]
pub async fn warn_message(
    ctx: types::AppContext<'_>,
    msg: serenity::Message,
) -> Result<(), types::Error> {
    warn(ctx, msg.author).await
}

async fn warn(
    ctx: types::AppContext<'_>,
    user: serenity::User,
) -> Result<(), types::Error> {
    let Some(form) = ({
        poise::execute_modal(ctx,
            Some(WarnModal{
                reason: format!("@{} adlı üyenin warn sebebi", user.name)
            }),
            None
        ).await?
    }) else {
        return Ok(())
    };

    let guild = ctx.guild_id().unwrap();
    let mut member = guild.member(ctx.http(), &user.id).await?;

    let warns = &ctx.data.config.warns;

    let warn = warns.iter().find_map(|role| {
        if !member.roles.contains(role) {
            Some(role)
        } else {
            None
        }
    });

    if let Some(role) = warn {
        member.add_role(ctx.http(), role).await?;

        ctx.send_message(format!("{} adlı üye uyarıldı", user)).await?;

        ctx.data.log_sys_with_embed(
            ctx.http(),
            format!("{} adlı üye uyarıldı", user),
            |c| {
                c.field("sebep", form.reason, true)
            }
        ).await?;

        if warns.iter().all(|r| member.roles.contains(r)) {
            ctx.send_message(
                "üye uyarı hakkını doldurduğundan yönetim cezaya karar veresiye kadar susturulmuştur"
            ).await?;

            member.disable_communication_until_datetime(
                ctx.http(),
                serenity::Timestamp::from(
                    chrono::Utc::now() + duration_str::parse("24d").unwrap()
                ),
            ).await?;

            log_sys!(ctx, "{} adlı üyenin cezasına karar veriniz here", user);
        }
    }

    return Ok(())
}
