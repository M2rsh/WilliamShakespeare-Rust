use serenity::builder::CreateApplicationCommand;
use serenity::model::prelude::interaction::application_command::CommandDataOption;
use tracing::{debug, error, info};

pub fn run(_options: &[CommandDataOption]) -> String {
    debug!("debug");
    error!("error");
    info!("info");
    "Tested".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("loggingtest").description("Test logs")
}