use sqlx;
use serenity::{framework::standard::Args, prelude::*};
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use crate::public::*;


#[command]
async fn own(ctx: &Context, msg: &Message) -> CommandResult {

    let id = &msg.author.id;
    if id.0 == 350750086357057537 {
        msg.channel_id.say(ctx, "Ok.").await?;
    
    } else {
        panic!("Nothing.");
    }
    Ok(())
}

#[command]
async fn prefix(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.reply(ctx, "No args was specified").await?;
        return Ok(());
    }

    let prefix = args.message();

    let pool = {
        let read = ctx.data.read().await;
        read.get::<DatabasePool>().unwrap().clone()
    };

    let guild = msg.guild_id.unwrap().0 as i64;
    let query = sqlx::query!("SELECT prefix FROM prefixes WHERE guild_id = $1")
        .fetch_optional(&pool)
        .boxed()
        .await?;

    Ok(())
}