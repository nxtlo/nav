mod commands;
mod public;

use std::{collections::{HashMap, HashSet}, env, error::Error, sync::Arc};
// use public::DatabasePool;
//use sqlx::postgres::PgPoolOptions;

// Serenity stuff
pub use serenity::framework::standard::macros::*;
use serenity::{framework::standard::*, model::id::UserId};
use serenity::framework::standard::help_commands::with_embeds;
use serenity::{
    async_trait,
    client::bridge::gateway::{ShardManager, GatewayIntents},
    framework::{
        StandardFramework,
        standard::macros::group,
    
    },
    http::Http,
    model::{
        id::GuildId,
        channel::Message,
        event:: ResumedEvent, gateway::Ready},
    prelude::{TypeMapKey, Context, EventHandler, Client},
};


use tokio::sync::Mutex;
use tracing::{error, info};
use tracing_subscriber::{
    FmtSubscriber,
    EnvFilter,
};

use commands::{
    meta::*,
    dev::*,
};

struct CommandCounter;
pub struct ShardManagerContainer;

impl TypeMapKey for CommandCounter {
    type Value = HashMap<String, u64>;
}

impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        info!("Connected to discord as {}", ready.user.name);
    }

    async fn resume(&self, _: Context, _: ResumedEvent) {
        info!("Resumed connection!");
    }

    async fn cache_ready(&self, _ctx: Context, _guilds: Vec<GuildId>) {
        info!("Caching is ready.");
    }

    async fn message(&self, _: Context, msg: Message) {
        if msg.author.bot {
            return;
        }
    }
}
#[group]
#[summary = "Meta commands for the bot."]
#[commands(ping, info, avatar)]
struct Meta;


#[group]
#[owners_only]
#[summary = "Commands for devs"]
#[commands(own)]
struct Dev;

#[help]
#[lacking_permissions = "Hide"]
async fn my_help(
    ctx: &Context,
    msg: &Message,
    args: Args,
    help_options: &'static HelpOptions,
    groups: &[&'static CommandGroup],
    owners: HashSet<UserId>,
) -> CommandResult {
    let _ = with_embeds(ctx, msg, args, help_options, groups, owners).await;
    Ok(())
}

// 

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {

    dotenv::dotenv().expect(".env file was not found.");

    let sub = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(sub).expect("Failed to load the logger.");

    // Load the token from .env

    let token = env::var("BOT_TOKEN")
        .expect("Token was not found in .env.");

    let prefix = env::var("PREFIX")
        .expect("Prefix was not found in .env");

    let http = Http::new_with_token(&token);

    let (owner, _bot_id) = match http.get_current_application_info().await {
        Ok(info) => {
            let mut owner = HashSet::new();
            owner.insert(info.owner.id);

            (owner, info.id)
        },
        Err(why) => panic!("Can't access the app info: {:?}", why),
    };

    let framework = StandardFramework::new()
        .configure(|c| c
                    .with_whitespace(true)
                    .on_mention(Some(_bot_id))
                   .owners(owner)
                   .prefix(&prefix))
        .group(&META_GROUP)
        .group(&DEV_GROUP)
        .help(&MY_HELP);

        let mut client = Client::builder(&token)
            .framework(framework)
            .event_handler(Handler)
            .intents({
                let mut intents = GatewayIntents::all();
                intents.remove(GatewayIntents::DIRECT_MESSAGE_TYPING);
                intents.remove(GatewayIntents::GUILD_MESSAGE_TYPING);
                intents
            })
            .await
            .expect("Err creating client");

    {   // let dsn = env::var("DSN_URL")?;
        let mut data = client.data.write().await;
        //let pool = PgPoolOptions::new().max_connections(20).connect(&dsn).await?;
        
       //data.insert::<DatabasePool>(pool);
        data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
    }



    let shard_manager = client.shard_manager.clone();

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.expect("No ctrl_c handler");
        shard_manager.lock().await.shutdown_all().await;
    });

    if let Err(why) = client.start().await {
        error!("Client err: {:?}", why);
    }

    Ok(())
}