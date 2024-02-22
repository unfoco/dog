use poise::serenity_prelude as serenity;

use crate::types;
use crate::util::macros::log_sys;
use crate::util::traits::{ExtendChannelId, ExtendContext};

#[derive(Debug, poise::Modal)]
#[name = "say"]
#[allow(dead_code)]
struct SayModal {
    #[paragraph]
    #[max_length = 1024]
    #[name = "mesaj"]
    message: String,
}

#[poise::command(slash_command, category = "admin", guild_only)]
pub async fn say(
    ctx: types::AppContext<'_>,
    channel: Option<serenity::ChannelId>,
) -> Result<(), types::Error> {
    let Some(form) = ({
        poise::execute_modal(
            ctx,
            Some(SayModal {
                message: "".to_string(),
            }),
            None,
        )
        .await?
    }) else {
        return Ok(());
    };

    let target = channel.unwrap_or(ctx.channel_id());

    let message = target
        .send_message_content(ctx.http(), form.message)
        .await?;

    log_sys!(
        ctx,
        "{} bot aracılığı ile {} mesajını gönderdi",
        ctx.author(),
        message.link()
    );
    Ok(())
}
