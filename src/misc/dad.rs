/*
 * dad.rs
 * LasangnaBot
 * use in response to fatherless behavior
 */

use crate::*;

pub async fn dad(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply_ping(&ctx, "I am so proud of you :)")
        .await
        .expect("unable to send message");
    Ok(())
}
