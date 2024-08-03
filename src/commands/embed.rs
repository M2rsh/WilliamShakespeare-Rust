use crate::{Context, Error};
use poise::{serenity_prelude::{Colour, CreateEmbed, CreateEmbedFooter}, CreateReply};

// This is so stupid
#[derive(Debug, poise::ChoiceParameter)]
pub enum ColourChoice {
    #[name = "White"] White,
    #[name = "Black"] Black,
    #[name = "Love"] Love,
    #[name = "Gold"] Gold,
    #[name = "Rose"] Rose,
    #[name = "Pine"] Pine,
    #[name = "Foam"] Foam,
    #[name = "Iris"] Iris,
}

impl Into<Colour> for ColourChoice {
    fn into(self) -> Colour {
        match self {
            ColourChoice::White => Colour(0xe0def4),
            ColourChoice::Black => Colour(0x191724),
            ColourChoice::Love => Colour(0xeb6f92),
            ColourChoice::Gold => Colour(0xf6c177),
            ColourChoice::Rose => Colour(0xebbcba),
            ColourChoice::Pine => Colour(0x31748f),
            ColourChoice::Foam => Colour(0x9ccfd8),
            ColourChoice::Iris => Colour(0xc4a7e7),
        }
    }
}

/// Create an embed
/// 
/// Cooldown: 3 seconds
#[poise::command(slash_command, user_cooldown = 3, rename = "embed", category = "idk")]
pub async fn run(
    ctx: Context<'_>,
    #[description = "Embed title / Big text"] 
    #[rename="title"]
        title_text: Option<String>,
    #[description = "Description / Medium text"]
    #[rename="description"]
        description_text: Option<String>,
    #[description = "Footer Text / Small text"] 
    #[rename="footer"]
        footer_text: Option<String>,
    #[description = "Embed colour"] 
    #[rename="colour"]
        colour: Option<ColourChoice>
) -> Result<(), Error> {
    if title_text.is_none() && description_text.is_none() && footer_text.is_none() {
        ctx.send(
            CreateReply::default()
                .content("You must provide at least one argument.")
                .ephemeral(true)
            ).await?;
        return Ok(())
    }

    //No way this is the best way to do this
    let footer = CreateEmbedFooter::new(if let Some(value) = footer_text { value } else { "".to_string() });

    let embed = CreateEmbed::default()
        .title(if let Some(value) = title_text { value } else { "".to_string() })
        .description(if let Some(value) = description_text { value } else { "".to_string() })
        .footer(footer)
        .colour(if let Some(value) = colour { value } else { ColourChoice::Black });

    ctx.send(CreateReply::default().embed(embed)).await?;
    Ok(())
}
