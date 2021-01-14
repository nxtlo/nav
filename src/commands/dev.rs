use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    CommandResult,
    macros::command,
};


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