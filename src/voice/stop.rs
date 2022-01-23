/*
 * stop.rs
 * LasangnaBoi 2022
 * stop playing and clear the queue
 */

use crate::*;
use voice::*;

///stop playing
pub async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue3 = handler.queue();
        let _ = queue3.stop();
        check_msg(msg.channel_id.say(&ctx.http, "Queue cleared.").await);

    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Must be in a voice channel to use that command!")
                .await,
        );
    }
    Ok(())
}
