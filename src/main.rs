#[macro_use]
extern crate lazy_static;

mod commands;

use config_file::FromConfigFile;
use poise::serenity_prelude as serenity;
use serde::Deserialize;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;
pub static VERSION: &str = "1.0.0";

#[derive(Deserialize)]
struct Config {
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
    static ref CONFIG: Config = Config::from_config_file("Settings.toml").unwrap();
}

#[tokio::main]
async fn main() {
    let intents = serenity::GatewayIntents::non_privileged();

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
                //poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                poise::builtins::register_in_guild(ctx, &framework.options().commands, 885976189049651200.into()).await?;
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
            println!("Logged in as {}, Version: {}", data_about_bot.user.name, VERSION);
        }
        serenity::FullEvent::InteractionCreate { interaction } => {
            let data = interaction.clone().command().unwrap();
            println!("Command `{}` used by `{} - {}`", data.data.name, data.user.name, data.user.id)
        }
        _ => {}
    }
    Ok(())
}