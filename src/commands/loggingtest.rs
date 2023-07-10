use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::InteractionResponseType;
use serenity::prelude::Context;
use tracing::{debug, error, info};

pub async fn run(
    _ctx: Context,
    _command: ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    debug!("debug");
    error!("error");
    info!("info");
    _command
        .create_interaction_response(&_ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("Logging tested :thumbsup:"))
        })
        .await
        .expect("TODO: panic message");
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("loggingtest").description("Test logs")
}
