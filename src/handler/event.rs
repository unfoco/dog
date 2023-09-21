use poise::serenity_prelude as serenity;

use crate::types;

pub async fn handle(
    _ctx: &serenity::Context,
    _framework: types::FrameworkContext<'_>,
    _data: &types::Data,
    event: &poise::Event<'_>,
) -> Result<(), types::Error> {
    println!("Got an event in event handler: {:?}", event.name());
    Ok(())
}
