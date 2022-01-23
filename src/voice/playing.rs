/*
 * playing.rs
 * LasangnaBoi 2022
 * get info from current song
 */

use crate::*;
use voice::*;

///get current song
pub async fn playing(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    //create manager
    let manager = songbird::get(ctx)
        .await
        .expect("songbird error")
        .clone();

    //get the queue
    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue1 = handler.queue();

        if queue1.is_empty() {
            check_msg(msg.channel_id.say(&ctx.http, "Nothing is being played!".to_string()).await);
            return Ok(());

        } else {
            let song = &queue1.current().unwrap().metadata().clone();

            //create embed
            let title = &song.title.as_ref().unwrap();
            let channel = &song.channel.as_ref().unwrap();
            let thumbnail = &song.thumbnail.as_ref().unwrap();
            let url = &song.source_url.as_ref().unwrap();
            let time = &song.duration.as_ref().unwrap();
            let minutes = time.as_secs()/60;
            let seconds = time.as_secs()-minutes*60;
            let duration = format!("{}:{:02}", minutes, seconds);
            let colour = Colour::from_rgb(149, 8, 2);
            assert_eq!(colour.r(), 149);
            assert_eq!(colour.g(), 8);
            assert_eq!(colour.b(), 2);
            assert_eq!(colour.tuple(), (149, 8, 2));
            let _ = msg.channel_id.send_message(&ctx.http, |m| {
                m.content("Current song:");
                m.embed(|e| {
                    e.title(title);
                    e.colour(colour);
                    e.description(channel);
                    e.field("duration: ", duration, false);
                    e.thumbnail(thumbnail);
                    e.url(url);
                    e
                })
            }).await;
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
