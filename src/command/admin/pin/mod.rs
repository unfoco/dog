use poise::serenity_prelude as serenity;
use ::serenity::json;

use crate::types;

mod create;
mod edit;

#[poise::command(context_menu_command = "msg pin", category = "admin", hide_in_help)]
pub async fn pin(
    ctx: types::AppContext<'_>,
    msg: serenity::Message,
) -> Result<(), types::Error> {
    let result = ctx.data.config.boards.iter()
        .map(|b| b.1.channel)
        .filter_map(|c| {
            if c == msg.channel_id.0 {
                Some(c)
            } else {
                None
            }
        }).next();

    return if let Some(_) = result {
        edit::handle(ctx, msg).await
    } else {
        create::handle(ctx, msg).await
    }
}

fn embed_to_json(embed: serenity::Embed) -> json::Value {
    let create: serenity::CreateEmbed = embed.into();
    json::Value::from(json::hashmap_to_json_map(create.0))
}
