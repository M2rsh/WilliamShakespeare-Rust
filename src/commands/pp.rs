use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, CreateEmbed};
use rand_distr::{Distribution, Normal};

// I am going to hell for this shit code

/// Measure pp size
#[poise::command(slash_command)]
pub async fn pp(
    ctx: Context<'_>,
    #[description = "Person"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());

    let normal = Normal::new(12.0, 2.0).unwrap();
    let pp_size = normal.sample(&mut rand::thread_rng());

    let mut embed = CreateEmbed::default();
    embed.description(format!(
        "{} pp size is {:.2}cm / {:.2}inch",
        u,
        pp_size,
        pp_size * 0.3937007874
    ));
    embed.colour(serenity::Colour::from(0xfc234b));

    ctx.send(|r| {
        r.embed(|e| {
            *e = embed.clone(); e
        })
    })
    .await?;
    Ok(())
}
