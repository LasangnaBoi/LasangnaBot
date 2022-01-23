/*
 * queue.rs
 * LasangnaBoi 2022
 * get the current queue
 */

use crate::*;
use voice::*;

///get the queue
pub async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue4 = handler.queue();

        if queue4.is_empty() {
            check_msg(msg.channel_id.say(&ctx.http, "The queue is empty!".to_string()).await);
            return Ok(());
        }
        
        //create embed
        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            let mut i = 10;
            if queue4.len() < 10 {
                i = queue4.len();
            }
            //color
            let colour = Colour::from_rgb(149, 8, 2);
            assert_eq!(colour.r(), 149);
            assert_eq!(colour.g(), 8);
            assert_eq!(colour.b(), 2);
            assert_eq!(colour.tuple(), (149, 8, 2));
            m.content("Current queue:");
            m.embed(|e| {
                e.title(format!("current length: {}", queue4.len()));
                e.color(colour);
                for i in 0..i {
                    let song = &queue4.current_queue().get(i).unwrap().metadata().clone();
                    let channel = &song.channel.as_ref().unwrap();
                    let title = &song.title.as_ref().unwrap();
                    //duration
                    let time = &song.duration.as_ref().unwrap();
                    let minutes = time.as_secs()/60;
                    let seconds = time.as_secs()-minutes*60;
                    let duration = format!("{}:{:02}", minutes, seconds);
                    let arg1 = format!("{}. {} | {}", i+1, title, channel);
                    e.field(arg1, duration, false);
                }
                e
            })
        }).await;
    } else {
       check_msg(
            msg.channel_id
                .say(&ctx.http, "Must be in a voice channel to use that command!")
                .await,
        );
    }
    Ok(())
}
