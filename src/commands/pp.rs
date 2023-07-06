use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::application_command::{ApplicationCommandInteraction};
use serenity::model::prelude::command::{CommandOptionType};
use serenity::model::prelude::interaction::application_command::{CommandDataOptionValue};
use serenity::model::prelude::InteractionResponseType;
use serenity::prelude::{Context, Mentionable};
use serenity::utils::Colour;
use rand::prelude::*;

pub async fn run(_ctx: Context, _command: ApplicationCommandInteraction) -> Result<(), serenity::Error> {
    let option = &_command.data.options
        .get(0)
        .expect("Expected user option")
        .resolved
        .as_ref()
        .expect("Expected user object");

    if let CommandDataOptionValue::User(user, _member) = option {
        let pp_size: f64 = thread_rng().gen::<f64>() * 35.0;
        let pp_size_inch: f64 = pp_size * 0.3937007874;

        _command.create_interaction_response(&_ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message|
                    message.embed(|embed| {
                        embed.description(format!("{} pp size is {:.2}cm - {:.2}inch", user.mention(), pp_size, pp_size_inch))
                            .colour(Colour::from_rgb(193, 188, 244))
                    })
                )
        }).await.expect("TODO: panic message");
    } else {
        _command.create_interaction_response(&_ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|message| message.content("ur gay idk"))
        }).await.expect("TODO: panic message");
    }

    Ok(())
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("pp").description("Measure users pp").create_option(|option| {
        option
            .name("user")
            .description("User to measure pp for")
            .kind(CommandOptionType::User)
            .required(true)
    })
}