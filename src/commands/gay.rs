use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, CreateEmbed};
use rand::Rng;

// I am going to hell for this shit code (again) (definetly not copied from pp command 100%)

/// Measure gayness
#[poise::command(slash_command)]
pub async fn gay(
    ctx: Context<'_>,
    #[description = "Person"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let mut embed = CreateEmbed::default();
    embed.description(format!(
        "{} is {:.2}% gay",
        u,
        rand::thread_rng().gen_range(0.0..100.0)
    ));
    embed.colour(serenity::Colour::from(0xf74598));

    ctx.send(|r| {
        r.embed(|e| {
            *e = embed.clone(); e
        })
    })
    .await?;
    Ok(())
}
