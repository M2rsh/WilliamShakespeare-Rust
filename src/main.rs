#[macro_use]
extern crate lazy_static;

use config_file::FromConfigFile;
use poise::serenity_prelude as serenity;
use serde::Deserialize;
use log::{info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;

mod commands;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub static VERSION: &str = "1.1.0";

#[derive(Deserialize)]
struct ConfigStruct {
    discord_token: String,
    rig_ids: Vec<u64>,
    emojis: Emojis,
}

#[derive(Deserialize)]
struct Emojis {
    big_brain: String,
    no_brain: String,
    just_brain: String,
    big_gay: Vec<String>,
    smol_gay: String,
    no_gay: String,
}

lazy_static! {
    static ref CONFIG: ConfigStruct = ConfigStruct::from_config_file("Settings.toml").unwrap();
}

#[tokio::main]
async fn main() {
    let intents = serenity::GatewayIntents::non_privileged();
    let stdout = ConsoleAppender::builder().build();

    let log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} - {m}{n}")))
        .build(format!("log/{:?}.log", chrono::offset::Local::now()))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("file", Box::new(log_file)))
        .logger(
            Logger::builder()
                .appender("file")
                .build("william_shakespeare", LevelFilter::Info),
        )
        .build(Root::builder().appender("stdout").build(LevelFilter::Error))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::help::run(),
                commands::info::run(),
                commands::serverinfo::run(),
                commands::pp::run(),
                commands::iq::run(),
                commands::gay::run(),
                commands::embed::run(),
            ],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                //poise::builtins::register_in_guild(ctx, &framework.options().commands, 885976189049651200.into()).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(&CONFIG.discord_token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();
}

async fn event_handler(
    _ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            info!(
                "Logged in as {}, Version: {}",
                data_about_bot.user.name, VERSION
            );
        }
        serenity::FullEvent::InteractionCreate { interaction } => {
            let data = interaction.clone().command().unwrap();
            info!(
                "Command `{}` used by `{} - {}`",
                data.data.name, data.user.name, data.user.id
            )
        }
        _ => {}
    }
    Ok(())
}
