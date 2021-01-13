use serenity::prelude::*;
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
async fn info(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id.say(&ctx.http, "I'm a discord bot was made by Fate using Rust and serenity").await?;

        Ok(())
}


#[command]
async fn avatar(ctx: &Context, msg: &Message) -> CommandResult {
    let name = &msg.author.name;
    let avatar = &msg.author.avatar_url();
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

    Ok(())
}