use poise::serenity_prelude as serenity;

use crate::types;

pub async fn send_dropdown(
    ctx: types::AppContext<'_>,
    text: impl Into<String>,
    values: Vec<impl Into<String>>,
) -> Result<Option<String>, types::Error> {
    let reply = ctx
        .send(|m| {
            m.content(text).components(|c| {
                c.create_action_row(|a| {
                    a.create_select_menu(|c| {
                        c.custom_id("dropdown");
                        c.options(|c| {
                            for value in values {
                                c.create_option(|c| {
                                    let value = value.into();
                                    c.label(value.clone()).value(value)
                                });
                            }
                            c
                        })
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
        .filter(move |mci| mci.data.custom_id == "dropdown")
        .await
    {
        reply.delete(types::Context::Application(ctx)).await?;
        return Ok(Some(mci.data.values[0].clone()));
    }

    reply.delete(types::Context::Application(ctx)).await?;
    Ok(None)
}
