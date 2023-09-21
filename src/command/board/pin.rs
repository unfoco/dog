use poise::serenity_prelude as serenity;
use serenity::json;

use crate::types;

#[poise::command(context_menu_command = "Pin to Board", required_permissions = "MANAGE_CHANNELS", hide_in_help)]
pub async fn pin(
    ctx: types::Context<'_>,
    #[description = "Pins message to a board"] msg: serenity::Message,
) -> Result<(), types::Error> {

    ctx.defer_ephemeral().await?;

    ctx.send(|m| {
        m.content("select a board to pin the message")
            .components(|c| {
                c.create_action_row(|a| {
                    a.create_select_menu(|c| {
                        c.custom_id("pin_to_board");
                        c.options(|c| {
                            for board in ctx.data().config.boards.iter() {
                                c.create_option(|c| {
                                    c.label(board.name.clone());
                                    c.value(board.room.clone())
                                });
                            }
                            c
                        })
                    })
                })
            })
    }).await?;

    while let Some(mci) =
        poise::serenity_prelude::CollectComponentInteraction::new(ctx.serenity_context())
            .timeout(std::time::Duration::from_secs(120))
            .filter(move |mci| mci.data.custom_id == "pin_to_board")
            .await
    {
        let url: String = ctx.data().config.boards.iter()
            .find_map(|x| {
                if x.room == mci.data.values[0] {
                    Some(x.webhook.clone())
                } else {
                    None
                }
            }).unwrap_or_default();

        let webhook = serenity::Webhook::from_url(ctx, &url).await?;

        let member = ctx.guild().unwrap().member(ctx, msg.author.id).await?;

        let name = member.display_name().clone();
        let avatar = member.avatar_url().unwrap_or_else(|| msg.author.avatar_url().unwrap());

        webhook.execute(ctx, true, |w| {
            w.username(&name).avatar_url(&avatar).content(msg.content.clone());

            for attachment in &msg.attachments {
                w.add_file(serenity::AttachmentType::Image(
                    url::Url::parse(&attachment.url).unwrap()
                ));
            }

            let mut embeds: Vec<json::Value> = msg.embeds.iter()
                .map(|e| super::embed_to_json(e.clone()))
                .collect();

            embeds.push(
                serenity::Embed::fake(|e| {
                    e.field("source", msg.link(), true)
                })
            );

            w.embeds(embeds)
        }).await.expect("unable to send message");

        mci.create_interaction_response(ctx, |c| {
            c.kind(serenity::InteractionResponseType::UpdateMessage)
        }).await?;

        mci.edit_original_interaction_response(ctx, |e| {
            e.content("pinned message to board")
                .components(|c| {c.0.clear(); c})
        }).await?;
    }
    Ok(())
}
