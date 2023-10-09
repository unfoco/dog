use poise::serenity_prelude as serenity;
use std::time::Duration;

mod command;
mod handler;
mod config;
mod types;

#[tokio::main]
async fn main() -> Result<(), types::Error> {
    env_logger::init();

    let config = config::Config::load()
        .expect("unable to load settings");

    poise::Framework::builder()
        .token(config.token.clone())
        .options(options())
        .intents(
            serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT,
        )
        .setup(move |ctx, ready, framework| Box::pin(
            handler::handle_setup(ctx, framework, ready, config)
        ))
        .run().await?;

    Ok(())
}

fn options() -> types::FrameworkOptions {
    poise::FrameworkOptions {
        commands: command::list(),
        prefix_options: poise::PrefixFrameworkOptions {
            edit_tracker: Some(poise::EditTracker::for_timespan(
                Duration::from_secs(3600)
            )),
            additional_prefixes: vec![],
            ..Default::default()
        },
        skip_checks_for_owners: false,
        on_error: |err| Box::pin(
            handler::error::handle(err)
        ),
        pre_command: |ctx| Box::pin(
            handler::command::pre_handle(ctx)
        ),
        post_command: |ctx| Box::pin(
            handler::command::post_handle(ctx)
        ),
        command_check: Some(|ctx| Box::pin(
            handler::command::run_handle(ctx)
        )),
        event_handler: |ctx, event, framework, data| Box::pin(
            handler::event::handle(ctx, framework, data, event)
        ),
        ..Default::default()
    }
}
