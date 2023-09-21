use serde_derive::Deserialize;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[derive(Deserialize)]
struct Data {
    keys: Keys,
}

#[derive(Deserialize)]
struct Keys {
    discord: String,
}

fn read_config() -> Data {
    let current_dir = std::env::current_dir().expect("current directory");
    let config_file = std::fs::read_to_string("./config.toml").expect("config file");
    let data: Data = toml::from_str(&config_file).expect("config file");
    data
}

#[group]
#[commands(ping)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::new()
        .configure(|c| c.prefix("#")) // set the bot's prefix to "#"
        .group(&GENERAL_GROUP);
    // Get the bot token from the environment
    let config = read_config();

    // Login with a bot token from the environment
    let token = config.keys.discord;
    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
