use poise::serenity_prelude as serenity;
use ::serenity::prelude::Mentionable;

use crate::types;
use crate::util::macros::log_sys;

const LIMIT: u64 = 100;

#[poise::command(slash_command, category = "admin")]
pub async fn purge(
    ctx: types::AppContext<'_>,
    #[max = 100] count: u64,
) -> Result<(), types::Error> {
    let messages = ctx.channel_id()
        .messages(ctx.http(), |g| {
            g.limit(count)
        })
        .await?;

    delete(ctx, messages).await
}

#[poise::command(context_menu_command = "msg purge", category = "admin", hide_in_help)]
pub async fn purge_message(
    ctx: types::AppContext<'_>,
    msg: serenity::Message,
) -> Result<(), types::Error> {
    let Some(form) = PurgeModal::execute(ctx).await else {
        return Ok(())
    };

    let Some((before, after)) = form.parse() else {
        let messages = ctx.channel_id()
            .messages(ctx.http(), |g| {
                g.after(&msg).limit(101)
            })
            .await?;

        if messages.len() <= LIMIT as usize {
            msg.delete(ctx.http()).await?;
            delete(ctx, messages).await?;
        }

        return Ok(())
    };

    if before == 0 && after == 0 {
        msg.delete(ctx.http()).await?;
        return Ok(())
    }

    let mut messages_before = ctx.channel_id()
        .messages(ctx.http(), |g| {
            g.before(&msg).limit(before)
        })
        .await?;
    let mut messages_after = ctx.channel_id()
        .messages(ctx.http(), |g| {
            g.before(&msg).limit(after)
        })
        .await?;

    messages_before.append(&mut messages_after);

    delete(ctx, messages_before).await
}

#[derive(Debug, poise::Modal)]
#[name = "purge"]
#[allow(dead_code)]
struct PurgeModal {
    #[name = "öncesi"]
    before: Option<String>,
    #[name = "sonrası"]
    after: Option<String>,
}

impl PurgeModal {
    async fn execute(ctx: types::AppContext<'_>) -> Option<Self> {
        poise::execute_modal(
            ctx,
            Some(Self{
                before: Some("0".to_string()),
                after: Some("0".to_string()),
            }),
            None
        ).await.ok()?
    }
    fn parse(&self) -> Option<(u64, u64)> {
        let b = self.before.as_ref().map(|b| b.parse().unwrap_or(0));
        let a = self.after.as_ref().map(|a| a.parse().unwrap_or(0));

        match (b, a) {
            (Some(mut b), Some(mut a)) => {
                if b > LIMIT {
                    b = LIMIT
                }
                if a > LIMIT {
                    a = LIMIT
                }
                Some((b, a))
            },
            _ => None,
        }
    }
}

async fn delete(
    ctx: types::AppContext<'_>,
    messages: Vec<serenity::Message>
) -> Result<(), types::Error> {
    ctx.channel_id()
        .delete_messages(ctx.http(), &messages)
        .await?;

    ctx.send(|c| {
        c.content(format!("{} mesaj kaldırıldı", messages.len()));
        c.ephemeral(true)
    }).await?;

    log_sys!(ctx,
        "{} {} kanalında {} mesaj kaldırdı",
        ctx.author(),
        ctx.channel_id().mention(),
        messages.len()
    );


    let log_member = ctx.data.config.logs.member;

    for message in messages {
        log_member.send_message(ctx.http(), |c| {
            c.content(format!(
                "{} kanalında {} tarafından gönderilen bir mesaj silindi",
                ctx.channel_id().mention(),
                message.author,
            ));

            for attachment in &message.attachments {
                c.add_file(serenity::AttachmentType::Image(
                    url::Url::parse(&attachment.url).unwrap()
                ));
            }

            if !message.content.is_empty() {
                c.embed(|c| {
                    c.description(message.content)
                });
            }
            c
        }).await?;
    }
    Ok(())
}