use poise::serenity_prelude as serenity;
use ::serenity::json;

use crate::types;

mod edit;
mod pin;

use edit::edit;
use pin::pin;

pub fn commands() -> types::CommandVec {
    return vec![edit(), pin()]
}

fn embed_to_json(embed: serenity::Embed) -> json::Value {
    let create: serenity::CreateEmbed = embed.into();
    json::Value::from(json::hashmap_to_json_map(create.0))
}
