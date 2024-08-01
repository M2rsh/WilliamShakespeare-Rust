#[macro_use]
extern crate lazy_static;

mod commands;
use config_file::FromConfigFile;
use poise::serenity_prelude as serenity;
use serde::Deserialize;

pub struct Data {} // User data, which is stored and accessible in all command invocations
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

#[derive(Deserialize)]
struct Config {
    discord_token: String,
    emojis: Emojis,
}

#[derive(Deserialize)]
struct Emojis {
    bigbrain: String,
    nobrain: String,
    justbrain: String,
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
                commands::pp::run(),
                commands::iq::run(),
            ],
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
