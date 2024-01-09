use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, CreateEmbed};
use rand_distr::{Distribution, Normal};

// I am going to hell for this shit code (again) (definetly not copied from pp command 100%)

/// Measure iq
#[poise::command(slash_command)]
pub async fn iq(
    ctx: Context<'_>,
    #[description = "Person"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let normal = Normal::new(100.0, 5.0).unwrap();
    let iq = normal.sample(&mut rand::thread_rng());

    let mut embed = CreateEmbed::default();
    embed.description(format!(
        "{} IQ is {:.2}",
        u,
        iq
    ));
    embed.colour(serenity::Colour::from(0xf7e245));

    ctx.send(|r| {
        r.embed(|e| {
            *e = embed.clone(); e
        })
    })
    .await?;
    Ok(())
}
