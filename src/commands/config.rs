use sqlx;
use serenity::{framework::standard::Args, prelude::*, utils::{ContentSafeOptions, content_safe}};
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};
use crate::public::*;


#[command]
#[required_permissions(MANAGE_GUILD)]
#[description("Set a custom prefix for the bot.")]
#[only_in(guilds)]
#[min_args(1)]
async fn prefix(ctx: &Context, msg: &Message, args: Args) -> CommandResult {
    if args.is_empty() {
        msg.reply(ctx, "No args was specified").await?;
        return Ok(());
    
    } else if args.len() > 5 {
        msg.reply(ctx, "Prefix cannot be longer then 5 charecters").await?;
        return Ok(())
    }

    let prefix = args.message();

    let pool = {
        let read = ctx.data.read().await;
        read.get::<DatabasePool>().unwrap().clone()
    };

    let guild = msg.guild_id.unwrap().0 as i64;

    let _query = sqlx::query!("SELECT prefix FROM guilds WHERE id = $1", guild)
        .fetch_optional(&pool)
        .boxed()
        .await?;

    if let None = _query {
        sqlx::query!(
            "INSERT INTO guilds(id, prefix) VALUES ($1, $2)",
            guild,
            &prefix
        )
        .execute(&pool)
        .await?;
    
    } else {
         sqlx::query!(
            "UPDATE guilds SET prefix = $2 WHERE id = $1", guild, &prefix)
        .execute(&pool)
        .await?;
    }

    let safe_content = ContentSafeOptions::default();
    let bad_msg = format!("Cahnged the prefix to {}", prefix);
    let final_result = content_safe(ctx, bad_msg, &safe_content).await;
    msg.reply(ctx, final_result).await?;

    Ok(())
}