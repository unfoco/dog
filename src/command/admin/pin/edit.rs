use poise::serenity_prelude as serenity;
use serenity::json;

use crate::types;

#[derive(Debug, poise::Modal)]
#[name = "pini düzenle"]
#[allow(dead_code)]
struct EditPinModal {
    #[paragraph]
    #[max_length = 1024]
    #[name = "pin notu"]
    note: Option<String>,
}

pub async fn handle(
    ctx: types::ContextApp<'_>,
    mut msg: serenity::Message,
) -> Result<(), types::Error> {
    let Some(webhook_id) = msg.webhook_id else {
        ctx.send(|c| {
            c.content("not a webhook message");
            c.ephemeral(true)
        })
        .await?;
        return Ok(());
    };

    let Some(embed) = msg.embeds.iter_mut().find_map(|e| {
        e.fields
            .iter()
            .find_map(|f| if f.name == "kaynak" { Some(()) } else { None })
            .map(|_| e)
    }) else {
        return Ok(());
    };

    let note_field: Option<(usize, &mut serenity::EmbedField)> = embed
        .fields
        .iter_mut()
        .enumerate()
        .find_map(|(i, f)| if f.name == "not" { Some((i, f)) } else { None });

    let note_value = if let Some(field) = &note_field {
        Some(field.1.value.clone())
    } else {
        None
    };

    let Some(form) = ({
        poise::execute_modal(
            ctx,
            Option::from(EditPinModal {
                note: note_value.clone(),
            }),
            None,
        )
        .await?
    }) else {
        return Ok(());
    };

    if note_value == form.note {
        return Ok(());
    }

    let webhook = webhook_id.to_webhook(ctx.http()).await?;

    if let Some((index, field)) = note_field {
        if let Some(new) = &form.note {
            field.value = new.clone();
        } else {
            embed.fields.remove(index);
        }
    } else {
        if let Some(new) = &form.note {
            embed
                .fields
                .push(serenity::EmbedField::new("not", new, true));
        }
    }

    let embeds: Vec<json::Value> = msg
        .embeds
        .iter()
        .map(|e| super::embed_to_json(e.clone()))
        .collect();

    webhook
        .edit_message(ctx.http(), msg.id, |e| e.embeds(embeds))
        .await?;

    ctx.log_sys_with_embed(
        format!("{} {} pinini düzenledi", ctx.author(), msg.link()),
        |c| {
            c.field("eski", note_value.unwrap_or_default(), true);
            c.field("yeni", form.note.unwrap_or_default(), true)
        },
    )
    .await?;

    Ok(())
}
