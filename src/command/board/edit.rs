use poise::serenity_prelude as serenity;
use serenity::json;

use crate::types;

#[derive(Debug, poise::Modal)]
#[name = "Edit Pin"]
#[allow(dead_code)]
struct EditPinModal {
    #[paragraph]
    #[max_length = 1024]
    #[name = "pin note"]
    note: String,
}

#[poise::command(context_menu_command = "Edit Pin Desc", required_permissions = "MANAGE_CHANNELS", hide_in_help)]
pub async fn edit(
    ctx: types::AppContext<'_>,
    #[description = "Pins message to a board"] mut msg: serenity::Message,
) -> Result<(), types::Error> {

    let Some(webhook_id) = msg.webhook_id else {
        ctx.defer_ephemeral().await?;
        ctx.reply("not a webhook message").await?;
        return Ok(())
    };

    let note = {
        let note = msg.embeds.iter()
            .filter(|e| {
                e.fields.iter()
                    .any(|f| f.name == "source")
            }).collect::<Vec<_>>();

        if note.is_empty() {
            ctx.defer_ephemeral().await?;
            ctx.reply("not a pin embed").await?;
            return Ok(())
        }

        if let Some(note_value) = note.iter()
            .find_map(|e| {
                e.fields.iter()
                    .find(|f| f.name == "note")
                    .map(|f| &f.value)
            }) {
            note_value.clone()
        } else {
            String::new()
        }
    };

    let response: Option<EditPinModal> =
        poise::execute_modal(
            ctx,
            Option::from(EditPinModal { note }),
            None
        ).await?;

    let Some(data) = response else {
        return Ok(())
    };

    let webhook = webhook_id.to_webhook(ctx.http()).await?;

    for embed in &mut msg.embeds {
        if let Some(field) = embed.fields.iter_mut()
            .find(|field| field.name == "note")
        {
            field.value = data.note.clone();
        } else {
            embed.fields.push(serenity::EmbedField::new(
                "note", data.note.clone(), true
            ));
        }
    }

    let embeds: Vec<json::Value> = msg.embeds.iter()
        .map(|e| super::embed_to_json(e.clone()))
        .collect();

    webhook.edit_message(ctx.http(), msg.id, |e| {
        e.embeds(embeds)
    }).await?;

    Ok(())
}
