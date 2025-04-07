use crate::types;

pub async fn handle(error: types::ErrorFramework<'_>) {
    match error {
        poise::FrameworkError::Setup { error, .. } => {
            panic!("failed to start bot: {:?}", error)
        },
        poise::FrameworkError::Command { error, ctx , .. } => {
            println!("error in command `{}`: {:?}", ctx.command().name, error,);
        }
        error => {
            if let Err(e) = poise::builtins::on_error(error).await {
                println!("error while handling error: {}", e)
            }
        }
    }
}
