use serenity::{framework::standard::Args, prelude::*};
use serenity::model::id::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};


#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "Pong!").await?;

    Ok(())
}

#[command]
#[aliases ("about")]
#[description("Gives you info about the bot.")]
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    let bot = &ctx.cache.current_user().await;
    let user_id = UserId::created_at(&bot.id);
    let bot_icon = &bot.avatar_url();

    let shard_count = &ctx.cache.shard_count().await;
    let cached_users = &ctx.cache.user_count().await;
    let cached_guilds = &ctx.cache.guilds().await.len();
    let unknown_users = &ctx.cache.unknown_members().await;
    
    msg.channel_id.send_message(ctx, |m|{
        m.embed(|e|{
            e.color(0xa078ff);
            e.field("BOT:", format!("Name: {}\nID: {}\nDiscriminator: {}", bot.name, bot.id, bot.discriminator), true);
            e.field("Created at:", user_id, false);
            e.field("Cache:", format!("Users: {}\nShards: {}\nGuilds: {}\nUnCached users: {}", cached_users, shard_count, cached_guilds, unknown_users), true);
            
            if let Some(i) = bot_icon {
                e.thumbnail(i);
            }
            e
        });

        m
    }).await?;
    
    Ok(())
}


#[command]
async fn avatar(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    
    let name = &msg.author.name;
    let avatar = &msg.author.avatar_url().clone();

    if args.is_empty() {
        msg.channel_id.send_message(ctx, |m|{
            m.embed(|e|{
    
                e.color(0x7bcde8);
                e.title(format!("{}", name));
                
                if let Some(ava) = avatar {
                    e.image(ava);
                }
                e
            });
    
            m
        }).await?;
    
    }

    Ok(())
}

#[command]
#[only_in(guilds)]
#[aliases ("si")]
#[description("Info about the current guild.")]
async fn serverinfo(ctx: &Context, msg: &Message) -> CommandResult {

    let stuff = &msg.guild_id.unwrap().clone();
    let guild = &ctx.cache.guild(stuff).await;

    msg.channel_id.send_message(ctx, |m|{
        m.embed(|e|{

            if let Some(x) = guild {
                e.field("ID", x.id, false);
                e.field("Name", format!("{}", x.name), false);
                e.field("Members", format!("{}", x.member_count), false);
                e.field("Region", format!("{}", x.region), false);
                e.field("Owner ID", format!("{}", x.owner_id), false);
                e.field("Boosters", format!("{}", x.premium_subscription_count), false);
                e.field("Features", format!("{:?}", x.features), true);

                if let Some(ava) = guild {
                    e.thumbnail(ava.icon_url().unwrap());
                }
            
            }

            e

        });

        m
    }).await?;

    Ok(())
}