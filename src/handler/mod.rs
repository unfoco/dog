use poise::serenity_prelude as serenity;

use crate::types;

pub mod command;
pub mod error;
pub mod event;

pub async fn handle_setup(
    ctx: &serenity::Context,
    _framework: &types::Framework,
    ready: &serenity::Ready,
    config: types::Config,
) -> Result<types::Data, types::Error> {
    let commands = commands();

    poise::builtins::register_in_guild(
        ctx, &commands.0[..], config.guild
    ).await?;
    poise::builtins::register_globally(
        ctx, &commands.1[..]
    ).await?;

    ctx.cache.set_max_messages(100);

    println!("logged in as {}", ready.user.name);

    Ok(types::Data {
        config
    })
}

// pity that discord limits menu commands to 5...
fn commands() -> (Vec<types::Command>, Vec<types::Command>) {
    let mut a = vec![];
    let mut b = vec![];

    for command in crate::command::list() {
        if command.context_menu_action.is_none() {
            a.push(command);
        } else {
            b.push(command);
        }
    }

    for _ in 1..=5 {
        let Some(command) = b.pop() else {
            continue
        };

        a.push(command)
    }

    (a, b)
}
