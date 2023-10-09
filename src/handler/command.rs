use crate::types;

pub async fn pre_handle(ctx: types::Context<'_>) {
    println!("Executing command {}...", ctx.command().qualified_name);
}

pub async fn post_handle(ctx: types::Context<'_>) {
    println!("Executed command {}!", ctx.command().qualified_name);
}

pub async fn run_handle(ctx: types::Context<'_>) -> Result<bool, types::Error> {
    let config = &ctx.data().config;

    Ok(match ctx.command().category {
        Some(category) => match category {
            "admin" => {
                let is = config.admins.contains(&ctx.author().id); if !is {
                    ctx.send(|c| {
                        c.content("bu komutu kullanmanız için yönetici olmanız lazım");
                        c.ephemeral(true)
                    }).await?;
                }
                is
            },
            &_ => true,
        }
        None => true,
    })
}
