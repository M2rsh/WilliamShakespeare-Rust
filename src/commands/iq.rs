use crate::{Context, Error, CONFIG};
use poise::{serenity_prelude::{self as serenity, CreateEmbed}, CreateReply};
use rand_distr::{Distribution, Normal};

/// IQ fun tingy something idk
#[poise::command(slash_command, user_cooldown=3, rename="iq", category = "Fun")]
pub async fn run(
    ctx: Context<'_>,
    #[description = "Person"] user: Option<serenity::User>,
) -> Result<(), Error> {

    let user = user.as_ref().unwrap_or_else(|| ctx.author());

    let iq: f64 = Normal::new(100.0, 25.0).unwrap()
                .sample(&mut rand::thread_rng());
    
    // IQ can go over 24 but it will just cause the colour to be brighter untill every number is 255 or over it
    // which will just convert it to u8 and max it  o 255
    let colour_thingy: f64 = iq/140.0; 
    let colour = serenity::Colour::from_rgb(
        (235.0*colour_thingy) as u8, 
        (188.0*colour_thingy) as u8, 
        (186.0*colour_thingy) as u8
    );

    let emoji;
    if iq >= 110.0 {
        emoji = &CONFIG.emojis.bigbrain;
    } else if 90.0 >= iq {
        emoji = &CONFIG.emojis.nobrain;
    } else {
        emoji = &CONFIG.emojis.justbrain;
    }

    let embed = CreateEmbed::default()
        .description(format!(
        "{} {} IQ is {:.2}",
        emoji,
        user,
        iq,
    ))
        .colour(colour);
     
    ctx.send(
        CreateReply{embeds: vec![embed], ..Default::default()}
    ).await?;
    Ok(())
}
