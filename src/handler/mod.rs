use poise::serenity_prelude as serenity;

use crate::{config, types};

pub mod event;
pub mod error;
pub mod command;

pub async fn handle_setup(
    ctx: &serenity::Context,
    framework: &types::Framework,
    ready: &serenity::Ready,
    config: config::Config
) -> Result<types::Data, types::Error> {
    poise::builtins::register_in_guild(
        ctx, &framework.options().commands,
        serenity::GuildId(1153049076644991047)
    ).await?;
    ctx.set_activity(serenity::Activity::watching("u")).await;
    //poise::builtins::register_globally(ctx, &framework.options().commands).await?;
    println!("Logged in as {}", ready.user.name);
    Ok(types::Data {
        config
    })
}
