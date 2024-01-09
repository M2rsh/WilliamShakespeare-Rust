use crate::{Context, Error};
use poise::serenity_prelude::{self as serenity, CreateEmbed};

// God fucking damn it there must be a better way to do embeds

/// Info about bot
#[poise::command(slash_command)]
pub async fn botinfo(
    ctx: Context<'_>
) -> Result<(), Error> {
    let mut embed = CreateEmbed::default();

    embed.description("I'm a bot made with Rust programming language [source code](https://github.com/M2rsh/WilliamShakespeare-Rust)
    
    more stuff to be added here in the future (probably not)");
    embed.colour(serenity::Colour::from(0x6436f9));

    // I have no idea
    // If there's a better way
    // To do this
    // But I'm too lazy
    // To search for it 
    //              - Ho Chi Minh 
    //                      (or someone idk)

    ctx.send(|r| { r.embed(|e| { *e = embed.clone(); e }) }).await?;

    // May our Lord and Saviour
    // Charles Christopher White Jr. 
    // (aka. MoistCr1TiKaL, Cr1TiKaL or penguinz0)
    //                                  (pronounced "penguin z zero")
    // Forgive me for this monstrosity 🙏

    Ok(())
}
