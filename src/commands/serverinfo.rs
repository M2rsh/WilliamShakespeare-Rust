use crate::{Context, Error,};
use poise::{serenity_prelude::CreateEmbed, CreateReply};

/// Server info
/// 
/// Cooldown: 5 seconds
#[poise::command(slash_command, user_cooldown = 5, rename = "serverinfo", category = "Utility")]
pub async fn run(
    ctx: Context<'_>,
) -> Result<(), Error> {
    let guild_data = ctx.guild().unwrap().clone();
    let embed = CreateEmbed::default()
        .field("Owner", format!("<@{}>", guild_data.owner_id.get()), true)
        .field("Members", guild_data.member_count.to_string(), true)
        .field("ID", guild_data.id.to_string(), true)
        .colour(0x9ccfd8);
    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
