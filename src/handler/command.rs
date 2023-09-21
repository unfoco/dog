use crate::types;

pub async fn pre_handle(ctx: types::Context<'_>) {
    println!("Executing command {}...", ctx.command().qualified_name);
}

pub async fn post_handle(ctx: types::Context<'_>) {
    println!("Executed command {}!", ctx.command().qualified_name);
}

pub async fn run_handle(_ctx: types::Context<'_>) -> Result<bool, types::Error> {
    Ok(true)
}
