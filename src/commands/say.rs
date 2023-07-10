use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::InteractionResponseType;
use serenity::prelude::Context;

pub async fn run(
    _ctx: Context,
    _command: ApplicationCommandInteraction,
) -> Result<(), serenity::Error> {
    let option = &_command
        .data
        .options
        .get(0)
        .expect("Expected string")
        .resolved
        .as_ref()
        .expect("Expected string");

    if let CommandDataOptionValue::String(_message) = option {
        _command
            .create_interaction_response(&_ctx.http, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.content(_message.to_string()))
            })
            .await
            .expect("TODO: panic message");
    }

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("say")
        .description("Say a message using bot")
        .create_option(|option| {
            option
                .name("message")
                .description("The message to send")
                .kind(CommandOptionType::String)
                .required(true)
        })
}
