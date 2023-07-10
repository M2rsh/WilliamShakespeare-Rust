use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::InteractionResponseType;
use serenity::prelude::Context;
use serenity::utils::Colour;

pub async fn run(_ctx: Context, _command: ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    _command.create_interaction_response(&_ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.embed(|embed| {
                embed
                    .description(format!("Discord bot written in Rust."))
                    .field(format!("Version: {}", env!("CARGO_PKG_VERSION")), "", true)
                    .field(format!("Authors: {}", env!("CARGO_PKG_AUTHORS").replace(":", ", ")), "", true)
                    .field(format!("Licence: {}", env!("CARGO_PKG_LICENSE")), "", true)
                    .field(format!("Repository: {}", env!("CARGO_PKG_REPOSITORY")), "", false)
                    .field(format!("Discord: https://discord.gg/dfKMTx9Eea"), "", true)
                    .colour(Colour::from_rgb(193, 188, 244))
            })
            )
    }).await.expect("TODO: panic message");
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("info").description("Info about the bot.")
}