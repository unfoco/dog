use crate::types;

#[poise::command(prefix_command, slash_command, required_permissions = "MANAGE_MESSAGES")]
pub async fn purge(
    ctx: types::Context<'_>,
    #[max = 50] count: u64,
) -> Result<(), types::Error> {

    if count > 50 {
        ctx.reply("you can't purge more than 50 messages at once").await?;
        return Ok(())
    }

    let messages = ctx
        .channel_id()
        .messages(ctx, |g| {
            g.limit(count)
        })
        .await?;

    ctx
        .channel_id()
        .delete_messages(ctx, messages)
        .await?;

    ctx.reply(format!("removed {} messages", count)).await?;
    Ok(())
}
