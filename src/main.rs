use poise::serenity_prelude as serenity;

mod command;
mod handler;
mod types;

#[tokio::main]
async fn main() -> Result<(), types::Error> {
    let config = types::Config::load()
        .expect("unable to load config");
    let token = config.token.clone();

    let framework = poise::Framework::builder()
        .options(options())
        .setup(move |ctx, ready, framework| {
            Box::pin(handler::handle_setup(ctx, framework, ready, config))
        })
        .build();

    let intents = serenity::GatewayIntents::all();
    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    
    client?
        .start()
        .await?;

    Ok(())
}

fn options() -> types::FrameworkOptions {
    poise::FrameworkOptions {
        commands: command::list(),
        command_check: Some(|ctx| {
            Box::pin(handler::command::handle(ctx))
        }),

        event_handler: |ctx, event, framework, data| {
            Box::pin(handler::event::handle(ctx, event, framework, data))
        },

        on_error: |err| {
            Box::pin(handler::error::handle(err))
        },

        ..Default::default()
    }
}
