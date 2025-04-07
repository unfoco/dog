use poise::serenity_prelude as serenity;
use serenity::Mentionable;

use crate::types;

const LIMIT: u8 = 100;

#[poise::command(slash_command, category = "admin", guild_only)]
pub async fn purge(
    ctx: types::ContextApp<'_>,
    #[max = 100] count: u8,
) -> Result<(), types::Error> {
    let messages = ctx
        .channel_id()
        .messages(
            ctx, serenity::GetMessages::new()
                .limit(count)
        )
        .await?;

    delete(ctx, messages).await
}

#[poise::command(
    context_menu_command = "msg purge",
    category = "admin",
    guild_only,
    hide_in_help
)]
pub async fn purge_message(
    ctx: types::ContextApp<'_>,
    msg: serenity::Message,
) -> Result<(), types::Error> {
    let Some(form) = PurgeModal::execute(ctx).await else {
        return Ok(());
    };

    let Some((before, after)) = form.parse() else {
        let mut messages = ctx
            .channel_id()
            .messages(
                ctx, serenity::GetMessages::new()
                    .after(&msg)
                    .limit(101)
            )
            .await?;

        if messages.len() <= LIMIT as usize {
            messages.push(msg);
            delete(ctx, messages).await?;
        }

        return Ok(());
    };

    if before == 0 && after == 0 {
        msg.delete(ctx).await?;
        return Ok(());
    }

    let mut messages_before = if before > 0 {
        ctx.channel_id()
            .messages(
                ctx, serenity::GetMessages::new()
                    .after(&msg)
                    .limit(before)
            )
            .await?
    } else {
        vec![]
    };
    let mut messages_after = if after > 0 {
        ctx.channel_id()
            .messages(
                ctx, serenity::GetMessages::new()
                    .after(&msg)
                    .limit(after)
            )
            .await?
    } else {
        vec![]
    };

    messages_before.append(&mut messages_after);
    messages_before.push(msg);

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
    async fn execute(ctx: types::ContextApp<'_>) -> Option<Self> {
        poise::execute_modal(
            ctx,
            Some(Self {
                before: Some("0".to_string()),
                after: Some("0".to_string()),
            }),
            None,
        )
        .await
        .ok()?
    }
    fn parse(&self) -> Option<(u8, u8)> {
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
            }
            _ => None,
        }
    }
}

async fn delete(
    ctx: types::ContextApp<'_>,
    messages: Vec<serenity::Message>,
) -> Result<(), types::Error> {
    ctx.channel_id()
        .delete_messages(ctx, &messages)
        .await?;

    ctx.send(
        poise::CreateReply::default()
            .content(format!("{} mesaj silindi", messages.len()))
            .ephemeral(true)
    ).await?;

    //log_sys!(
    //    ctx,
    //    "{} {} kanalında {} mesaj silindi",
    //    ctx.author(),
    //    ctx.channel_id().mention(),
    //    messages.len()
    //);

    let log_member = ctx.data.config.logs.member;

    for message in messages {
        if message.author.bot {
            continue;
        }

        //log_member
        //    .send_message(ctx, |c| {
        //        c.add_embed(|c| {
        //            c.description(format!(
        //                "{} kanalında {} tarafından gönderilen bir mesaj kaldırıldı",
        //                ctx.channel_id().mention(),
        //                message.author,
        //            ))
        //        });

        //        if !message.content.is_empty() {
        //            c.add_embed(|c| c.description(message.content));
        //        }

        //        for attachment in &message.attachments {
        //            c.add_file(serenity::AttachmentType::Image(
        //                url::Url::parse(&attachment.url).unwrap(),
        //            ));
        //        }

        //        c
        //    })
        //    .await?;
    }
    Ok(())
}
