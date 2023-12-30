use crate::{Context, Error};
use regex::Regex;

fn sanitize_message(message: &str) -> String {
    let pattern_str = r#"<@!?(\d+)>|@(everyone|here|&\d+)"#;
    let pattern = Regex::new(pattern_str).expect("Invalid regex pattern");

    let result = pattern.replace_all(message, |caps: &regex::Captures| {
        let p1 = caps.get(1).map_or("", |m| m.as_str());
        //let p2 = caps.get(2).map_or("", |m| m.as_str());
        
        if !p1.is_empty() {
            caps[0].to_string()
        } else {
            "@lol no".to_string()
        }
    });

    result.into_owned()
}

/// Say something using the bot
#[poise::command(slash_command)]
pub async fn say(
        ctx: Context<'_>, 
        #[description = "Text"] text: String,
    ) -> Result<(), Error> {
    ctx.say(format!("{}", sanitize_message(&text))).await?;
    Ok(())
}