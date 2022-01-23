/*
 * skip.rs
 * LasangnaBoi 2022
 * skip the current song
 */

use crate::*;
use voice::*;

///skip the track
pub async fn skip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue2 = handler.queue();

        if queue2.is_empty() {
            check_msg(msg.channel_id.say(&ctx.http, "The queue is empty!".to_string()).await);
            return Ok(());

        } else {
            let _ = queue2.skip();

            check_msg(
                msg.channel_id
                    .say(&ctx.http, "Song skipped!")
                    .await,
            );
        }
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Must be in a voice channel to use that command!")
                .await,
        );
    }
    Ok(())
}
