use poise::serenity_prelude as serenity;
use serenity::FullEvent;

use crate::types;

mod message_delete;
mod message_update;
mod reaction_remove;
mod guild_member_add;
mod message;

pub async fn handle(
    ctx: &serenity::Context,
    event: &FullEvent,
    framework: types::ContextFramework<'_>,
    data: &types::Data,
) -> Result<(), types::Error> {
    match event {
        //FullEvent::MessageDelete {
        //    channel_id,
        //    deleted_message_id,
        //    guild_id,
        //} => {
        //    message_delete::handle(
        //        ctx, framework, data, channel_id, deleted_message_id, guild_id
        //    ).await?
        //}

        FullEvent::MessageUpdate {
            old_if_available,
            new,
            event,
        } => {
            message_update::handle(
                ctx, framework, data, old_if_available, new, event
            ).await?
        },

        FullEvent::ReactionRemove { removed_reaction } => {
            reaction_remove::handle(
                ctx, framework, data, removed_reaction
            ).await?
        }

        FullEvent::GuildMemberAddition { new_member } => {
            guild_member_add::handle(
                ctx, framework, data, new_member
            ).await?
        }

        FullEvent::Message { new_message } => {
            message::handle(
                ctx, framework, data, new_message
            ).await?
        }

        _ => {}
    }
    Ok(())
}
