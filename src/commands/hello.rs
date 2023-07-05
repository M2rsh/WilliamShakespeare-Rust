use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::InteractionResponseType;
use serenity::prelude::Context;

pub async fn run(_ctx: Context, _command: ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    _command.create_interaction_response(&_ctx.http, |response| {
        response
            .kind(InteractionResponseType::ChannelMessageWithSource)
            .interaction_response_data(|message| message.content("Hello from Rust!"))
    }).await.expect("TODO: panic message");
    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("hello").description("A ping command")
}