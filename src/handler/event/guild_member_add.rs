use poise::serenity_prelude as serenity;

use crate::types;

pub async fn handle(
    ctx: &serenity::Context,
    _framework: types::ContextFramework<'_>,
    data: &types::Data,
    new_member: &serenity::Member, // Change here
) -> Result<(), types::Error> {
    let role = data.config.roles.default;

    if let Err(err) = new_member.add_role(&ctx.http, role).await {
        eprintln!("error adding role: {:?}", err);
    } else {
        println!("successfully added role to: {:?}", new_member.user.name);
    }

    Ok(())
}
