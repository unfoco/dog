use poise::serenity_prelude as serenity;
use crate::types;

pub async fn handle(
    ctx: &serenity::Context,
    _framework: types::FrameworkContext<'_>,
    data: &types::Data,
    new_member: &serenity::Member, // Change here
) -> Result<(), types::Error> {
    println!("New member joined: {:?}", new_member.user.name);
    let mut member = new_member.clone();
    let role = data.config.autorole;

    if let Err(err) = member.add_role(&ctx.http, role).await {
        eprintln!("Error adding role: {:?}", err);
    } else {
        println!("Successfully added role to: {:?}", new_member.user.name);
    }

    Ok(())
}
