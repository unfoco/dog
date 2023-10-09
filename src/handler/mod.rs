use poise::serenity_prelude as serenity;

use crate::{config, types};

pub mod event;
pub mod error;
pub mod command;

pub async fn handle_setup(
    ctx: &serenity::Context,
    _framework: &types::Framework,
    ready: &serenity::Ready,
    config: config::Config
) -> Result<types::Data, types::Error> {
    let mut slash = vec![];
    let mut menu = vec![];

    for command in crate::command::list() {
        if command.context_menu_action.is_none() {
            slash.push(command);
        } else {
            menu.push(command);
        }
    }

    let guild = serenity::GuildId(1153049076644991047);

    poise::builtins::register_in_guild(
        ctx, &slash,
        guild
    ).await?;

    poise::builtins::register_in_guild(
        ctx, &menu[..5],
        guild
    ).await?;

    poise::builtins::register_globally(
        ctx, &menu[5..]
    ).await?;

    ctx.cache.set_max_messages(100);

    ctx.set_activity(serenity::Activity::watching("u")).await;
    //poise::builtins::register_globally(ctx, &framework.options().commands).await?;
    println!("Logged in as {}", ready.user.name);
    Ok(types::Data {
        config
    })
}
