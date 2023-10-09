use poise::serenity_prelude as serenity;
use ::serenity::prelude::Mentionable;
use ::serenity::json;

use crate::types;

pub async fn handle(
    ctx: types::AppContext<'_>,
    msg: serenity::Message,
) -> Result<(), types::Error> {

    ctx.send(|m| {
        m.content("mesajı pinlemek istediğiniz panoyu seçiniz")
            .components(|c| {
                c.create_action_row(|a| {
                    a.create_select_menu(|c| {
                        c.custom_id("pin_to_board");
                        c.options(|c| {
                            for board in ctx.data().config.boards.keys() {
                                c.create_option(|c| {
                                    c.label(board).value(board)
                                });
                            }
                            c
                        })
                    })
                })
            });
        m.ephemeral(true)
    }).await?;

    while let Some(mci) =
        serenity::CollectComponentInteraction::new(ctx.serenity_context())
            .author_id(ctx.author().id)
            .channel_id(ctx.channel_id())
            .timeout(std::time::Duration::from_secs(120))
            .filter(move |mci| mci.data.custom_id == "pin_to_board")
            .await
    {
        let board = ctx.data().config.boards
            .get(&mci.data.values[0])
            .cloned()
            .unwrap();

        let webhook = board.webhook(ctx.http()).await?;

        let member = ctx.guild().unwrap().member(ctx.http(), msg.author.id).await?;

        let name = member.display_name().clone();
        let avatar = member.avatar_url().unwrap_or_else(|| msg.author.avatar_url().unwrap());

        webhook.execute(ctx.http(), true, |w| {
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
                    e.field("kaynak", msg.link(), true)
                })
            );

            w.embeds(embeds)
        }).await.expect("unable to send message");

        mci.create_interaction_response(ctx.http(), |c| {
            c.kind(serenity::InteractionResponseType::UpdateMessage)
        }).await?;

        mci.delete_original_interaction_response(ctx.http()).await?;

        msg.reply(
            ctx.http(),
            format!(
                "mesaj {} adlı panoya pinlendi",
                board.channel.mention(),
            )
        ).await?;

        ctx.data.log_sys(
            ctx.http(),
            format!(
                "{} {} mesajını {} adlı panoya pinledi",
                ctx.author(),
                msg.link(),
                board.channel.mention(),
            )
        ).await?;
    }
    Ok(())
}
