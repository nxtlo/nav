use serenity::{framework::standard::Args, prelude::*};
use serenity::model::id::GuildId;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};

use crate::public::DatabasePool;
use sqlx;


#[command]
#[only_in(guilds)]
#[description("Gets you the content of the tag by its name.")]
#[sub_commands(create)]
async fn tag(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    let pool = {
        let data_read = ctx.data.read().await;
        data_read.get::<DatabasePool>().unwrap().clone()
    };

    let tag_name = args.single::<String>().unwrap();
    let guild_id = msg.guild_id.unwrap().0 as i64;

    let result = sqlx::query!(
        "SELECT content FROM tags WHERE guild_id = $1 AND name = $2", &guild_id, &tag_name
    )
        .fetch_optional(&pool)
        .boxed()
        .await?;

    if let None = result {
        msg.reply(ctx, format!("Tag name {} was not found", tag_name)).await?;
        return Ok(());

    } else {
        msg.reply(ctx, format!("{:#?}", result.unwrap())).await?;
        return Ok(());
    }

}


#[command]
#[aliases("new", "add")]
#[description("Creates a new tag.")]
async fn create(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {

    if args.is_empty() {
        msg.reply(ctx, "Tag content cannot be empty.").await?;
        return Ok(());
    }

    let tag = args.single::<String>().unwrap();
    let content = args.rest().to_string();

    let pool = {
        let read_data = ctx.data.read().await;
        read_data.get::<DatabasePool>().unwrap().clone()
    };

    let guild_id = msg.guild_id.unwrap().0 as i64;

    let query = sqlx::query!("SELECT name FROM tags WHERE name = $1 AND guild_id = $2", tag, guild_id)
        .fetch_optional(&pool)
        .boxed()
        .await?;

    if let None = query {
        sqlx::query!(
            "INSERT INTO tags (guild_id, owner, name, content) VALUES ($1, $2, $3, $4)",
            msg.guild_id.unwrap_or(GuildId(0)).0 as i64,
            msg.author.id.0 as i64,
            tag,
            content,
        )
        .execute(&pool)
        .await?;
    
        msg.reply(ctx, format!("Ok, saved note `{}`", tag)).await?;

    } else {
        msg.reply(ctx, format!("Tag name `{}` already taken.", tag)).await?;
        return Ok(());

    }

    Ok(())

}