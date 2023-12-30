mod commands;

use poise::serenity_prelude as serenity;
use poise::Event;

use log::info;
use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::rolling_file::RollingFileAppender;
use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Logger, Root};

struct Data {} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

#[tokio::main]
async fn main() {
    // Log4rs cofig and setup
    let window_size = 24;
    let fixed_window_roller = FixedWindowRoller::builder().build("log/log-{}.log", window_size).unwrap();

    let size_limit = 512 * 1024; // 512KB
    let size_trigger = SizeTrigger::new(size_limit);
    let compound_policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(fixed_window_roller));

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%+)(utc)} {h({l})} {m}{n}")))
        .build();

    let info_file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d(%+)(utc)} [{f}:{L}] {h({l})} {M}:{m}{n}")))
        .build("log/log.log", Box::new(compound_policy))
        .unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("info_file", Box::new(info_file)))
        .logger(Logger::builder()
            .appender("info_file")
            .appender("stdout")
            .additive(false)
            .build("william_shakespeare_rust", LevelFilter::Info))
        .build(Root::builder().appender("stdout").build(LevelFilter::Warn))
        .unwrap();

    let _handle = log4rs::init_config(config).unwrap();

    // Bot initialization
    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::say::say()],
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            ..Default::default()
        })
        .token(std::env::var("DISCORD_TOKEN").expect("missing DISCORD_TOKEN"))
        .intents(serenity::GatewayIntents::non_privileged())
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        });

    framework.run().await.unwrap();
}

async fn event_handler(
    _ctx: &serenity::Context,
    event: &Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    match event {
        Event::Ready { data_about_bot, .. } => {
            info!("Logged in as {}", data_about_bot.user.name);
        }
        Event::InteractionCreate { interaction, .. } => {
            let data = interaction.clone().application_command().unwrap();
            let mut options: String = String::new();
            
            // I am very stupid
            for i in data.data.options {
                if let Some(data_from_value) = i.value {
                    if !options.is_empty() {
                        options.push_str(", ");
                    }
                    options.push_str(&format!("{}: {}", &i.name, data_from_value));
                }
            }

            info!(
                "{} ran command '{}' with options '{}'",
                data.user.name, data.data.name, options
            )
        }
        _ => {}
    }
    Ok(())
}
