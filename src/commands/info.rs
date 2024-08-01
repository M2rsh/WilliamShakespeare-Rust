use crate::{Context, Error};

/// Bot info command
/// 
/// Usage: /info 
/// test test
/// more test
/// plz work
#[poise::command(slash_command, user_cooldown=5, rename="info", category = "Utility")]
pub async fn run(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let response = "Hello World!";
    ctx.say(response).await?;
    Ok(())
}
