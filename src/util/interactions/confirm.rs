use poise::serenity_prelude as serenity;

use crate::types;

pub async fn send_confirm(
    ctx: types::AppContext<'_>,
    text: impl Into<String>,
) -> Result<bool, types::Error> {
    let reply = ctx
        .send(|m| {
            m.content(text).components(|c| {
                c.create_action_row(|c| {
                    c.create_button(|c| {
                        c.custom_id("button_yes");
                        c.label("evet")
                    });
                    c.create_button(|c| {
                        c.custom_id("button_no");
                        c.label("hayır")
                    })
                })
            });
            m.ephemeral(true)
        })
        .await?;

    while let Some(mci) = serenity::CollectComponentInteraction::new(ctx.serenity_context())
        .author_id(ctx.author().id)
        .channel_id(ctx.channel_id())
        .message_id(reply.message().await.unwrap().id)
        .timeout(std::time::Duration::from_secs(60))
        .await
    {
        reply.delete(types::Context::Application(ctx)).await?;
        return Ok(match mci.data.custom_id.as_str() {
            "button_yes" => true,
            _ => false,
        });
    }

    reply.delete(types::Context::Application(ctx)).await?;
    Ok(false)
}
