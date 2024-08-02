use crate::{Context, Error, VERSION};
use poise::{
    serenity_prelude::{CreateActionRow, CreateButton, CreateEmbed},
    CreateReply,
};

/// Bot info
///
/// Cooldown: 5 seconds
#[poise::command(
    slash_command,
    user_cooldown = 5,
    rename = "info",
    category = "Utility"
)]
pub async fn run(ctx: Context<'_>) -> Result<(), Error> {
    let embed = CreateEmbed::default()
        .description("I'm a Discord bot written in Rust. (Work in Progress)")
        .field("Version", VERSION, true)
        .field("Contributors", "<@846298981797724161>", true)
        .colour(0x31748f);
    let action_row = CreateActionRow::Buttons(vec![
        CreateButton::new_link("https://github.com/M2rsh/WilliamShakespeare-Rust")
            .label("Source code"),
        CreateButton::new_link("https://discord.gg/dfKMTx9Eea")
            .label("Discord Server")
    ]);
    let response = CreateReply::default()
        .embed(embed)
        .components(vec![action_row]);
    ctx.send(response).await?;
    Ok(())
}
