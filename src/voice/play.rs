/*
 * play.rs
 * LasangnaBoi 2022
 * play a song from YouTube
 */

use crate::*;
use voice::*;

///play song from youtube
pub async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    let query = &msg.content[5..];
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    check_msg(
        msg.channel_id
        .say(&ctx.http, format!("searching YouTube for{}...", query))
        .await);

    //create manager
    let manager = songbird::get(ctx)
        .await
        .expect("songbird error")
        .clone();

    //create audio source
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        //get source from YouTube
        let source = match ytdl_search(query).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                check_msg(msg.channel_id.say(&ctx.http, "Error sourcing ffmpeg").await);

                return Ok(());
            },
        };

        //create embed
        let title = source.metadata.title.as_ref().unwrap();
        let channel = source.metadata.channel.as_ref().unwrap();
        let thumbnail = source.metadata.thumbnail.as_ref().unwrap();
        let url = source.metadata.source_url.as_ref().unwrap();
        let time = source.metadata.duration.as_ref().unwrap();
        let minutes = time.as_secs()/60;
        let seconds = time.as_secs()-minutes*60;
        let duration = format!("{}:{:02}", minutes, seconds);
        let colour = Colour::from_rgb(149, 8, 2);
        assert_eq!(colour.r(), 149);
        assert_eq!(colour.g(), 8);
        assert_eq!(colour.b(), 2);
        assert_eq!(colour.tuple(), (149, 8, 2));
        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            m.content("Added to queue:");
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

        //add to queue
        let (mut audio, _) = create_player(source);
        audio.set_volume(0.5);
        handler.enqueue(audio);

    //if not in a voice channel
    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Must be in a voice channel to use that command!")
                .await,
        );
    }
    Ok(())
}
