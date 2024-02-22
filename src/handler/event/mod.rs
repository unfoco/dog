use poise::serenity_prelude as serenity;
use poise::Event;

use crate::types;

mod message_delete;
mod message_update;
mod reaction_remove;

pub async fn handle(
    ctx: &serenity::Context,
    framework: types::FrameworkContext<'_>,
    data: &types::Data,
    event: &Event<'_>,
) -> Result<(), types::Error> {
    match event {
        Event::MessageDelete {
            channel_id,
            deleted_message_id,
            guild_id,
        } => {
            message_delete::handle(
                ctx,
                framework,
                data,
                channel_id,
                deleted_message_id,
                guild_id,
            )
            .await?
        }
        Event::MessageUpdate {
            old_if_available,
            new,
            event,
        } => message_update::handle(ctx, framework, data, old_if_available, new, event).await?,
        Event::ReactionRemove { removed_reaction } => {
            reaction_remove::handle(ctx, framework, data, removed_reaction).await?
        }
        _ => {}
    }
    Ok(())
}
