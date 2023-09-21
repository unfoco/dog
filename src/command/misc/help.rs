use crate::types;

#[poise::command(prefix_command, track_edits, slash_command)]
pub async fn help(
    ctx: types::Context<'_>,
    #[description = "Specific command to show help about"]
    #[autocomplete = "poise::builtins::autocomplete_command"]
    command: Option<String>,
) -> Result<(), types::Error> {
    poise::builtins::help(
        ctx,
        command.as_deref(),
        poise::builtins::HelpConfiguration {
            ..Default::default()
        },
    )
        .await?;
    Ok(())
}
