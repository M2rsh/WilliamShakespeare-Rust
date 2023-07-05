mod commands;

use std::{env};

use serenity::async_trait;
use serenity::model::application::command::Command;
use serenity::model::application::interaction::{Interaction, InteractionResponseType};
use serenity::model::gateway::Ready;
use serenity::model::event::ResumedEvent;
use serenity::prelude::*;
use tracing::{debug, error, info, instrument};
use tracing_subscriber::{EnvFilter, prelude::*};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
        let global_commands= Command::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::hello::register(command))
                .create_application_command(|command| commands::loggingtest::register(command))
        }).await;
        if let Ok(i) = global_commands {
            for command in i {
                info!("Registered command: {:#?}", command.name)
            }
        }
    }

    #[instrument(skip(self, _ctx))]
    async fn resume(&self, _ctx: Context, resume: ResumedEvent) {
        debug!("Resumed; trace: {:?}", resume.trace);
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            info!("Received command interaction: {:#?}", command.user.name);

            match command.data.name.as_str() {
                "hello" => commands::hello::run(ctx, command).await,
                "loggingtest" => commands::loggingtest::run(ctx, command).await,
                _ => command.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content("Failed to handle command"))
                }).await
            }.expect("TODO: panic message");
        }
    }
}

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::never("/home/m2rsh/Documents/IdeaProjects/WilliamShakespeare-Rust/logs", "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with(
            tracing_subscriber::fmt::layer()
                  .compact()
        )
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(non_blocking)
                .with_ansi(false)
                .compact()
        )
        .init();

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let mut client = Client::builder(token, GatewayIntents::empty())
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        error!("Client error: {:?}", why);
    }
}
