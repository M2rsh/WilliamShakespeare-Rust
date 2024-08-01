use crate::{Context, Error};
use poise::{serenity_prelude::{self as serenity, CreateEmbed}, CreateReply};
use rand_distr::{Distribution, Normal};

/// Measure pp size
#[poise::command(slash_command, user_cooldown=3, rename="pp", category = "Fun")]
pub async fn run(
    ctx: Context<'_>,
    #[description = "Person"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let user = user.as_ref().unwrap_or_else(|| ctx.author());

    let pp_size: f64 = Normal::new(12.0, 8.0).unwrap()
                .sample(&mut rand::thread_rng());
    
    //PP size can go over 24 but it will just cause the colour to be brighter untill every number is 255 or over it
    //which will just convert it to u8 and max it to 255
    let colour_thingy: f64 = pp_size/22.0; 
    let colour = serenity::Colour::from_rgb(
        (196.0*colour_thingy) as u8, 
        (167.0*colour_thingy) as u8, 
        (231.0*colour_thingy) as u8
    );

    let embed = CreateEmbed::default()
        .description(format!(
        "{} pp size is {:.2}cm / {:.2}inch",
        user,
        pp_size,
        pp_size * 0.3937007874
    ))
        .colour(colour);
     
    ctx.send(
        CreateReply{embeds: vec![embed], ..Default::default()}
    ).await?;
    Ok(())
}
