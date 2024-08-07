use crate::{Context, Error, CONFIG};
use poise::{serenity_prelude::{self as serenity, CreateEmbed}, CreateReply};
use rand::seq::SliceRandom;
use rand::Rng;

/// Gay metre
/// 
/// Cooldown: 3 seconds
#[poise::command(slash_command, user_cooldown = 3, rename = "gay", category = "Fun")]
pub async fn run(
    ctx: Context<'_>,
    #[description = "Person"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());

    let gay: f64 = if CONFIG.ultra_gay.contains(&user.id.get()) {
        rand::thread_rng().gen_range(90.0..10000.0)
    } else {
        rand::thread_rng().gen_range(0.0..100.0)
    };

    let colour_thingy: f64 = gay / 100.0;
    let colour = serenity::Colour::from_rgb(
        (197.0 * colour_thingy) as u8,
        (167.0 * colour_thingy) as u8,
        (231.0 * colour_thingy) as u8,
    );
    let emoji = if gay >= 66.6 {
        if let Some(value) = &CONFIG.emojis.big_gay.choose(&mut rand::thread_rng()) { value } else { "" }
    } else if 33.3 >= gay {
        &CONFIG.emojis.no_gay
    } else {
        &CONFIG.emojis.smol_gay
    };

    let embed = CreateEmbed::default()
        .description(format!("{} {} is {:.2}% gay", emoji, user, gay))
        .colour(colour);

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
