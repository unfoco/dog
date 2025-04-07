use crate::types;

pub async fn handle(ctx: types::Context<'_>) -> Result<bool, types::Error> {
    let Some(category) = &ctx.command().category else {
        return Ok(true);
    };

    if category == "admin" {
        let admins = &ctx.data().config.admins;

        if admins.contains(&ctx.author().id) {
            return Ok(true);
        }

        ctx.send(
            poise::CreateReply::default()
                .content("bu komutu kullanmanız için yönetici olmanız lazım")
                .ephemeral(true)
        ).await?;

        return Ok(false);
    }

    Ok(true)
}
