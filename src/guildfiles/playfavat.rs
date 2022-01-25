/*
 * playfavat.rs
 * LasangnaBoi 2022
 * play favorited song at specified index
 */

use crate::*;
use guildfiles::*;

///play song from favorites
pub async fn playfavat(ctx: &Context, msg: &Message) -> Result<()> {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let favpath = format!("./guild_files/{}", guild_id);
    let size: i32 =  read_dir(&favpath).unwrap().count().try_into().expect("failed to parse");
    let query: i32 = msg.content[11..].chars().as_str().parse::<i32>().expect("error parsing");
    if size == 0 {
        msg.reply(ctx, "No songs are saved to favorites!".to_string())
            .await
            .expect("guild data has not been initialized");
        return Ok(());
    }
    if query > size || query == 0 {
        msg.reply(ctx, "not a valid track!")
            .await
            .expect("error sending message");
        return Ok(());
    }
    //create manager
    let manager = songbird::get(ctx)
        .await
        .expect("songbird error")
        .clone();

    //create audio source
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let song = read_dir(&favpath).expect("failed to get path")
            .nth((query-1).try_into().unwrap());

        let path = song.unwrap().unwrap().path().to_str().unwrap().to_string();
        if let Ok(mut lines) = read_lines(format!("{}/data.txt", &path)) {

            //get source from YouTube
            let url = &lines.nth(0).expect("failed to read line").expect("failes to read line");
            let source = match ytdl_search(url).await {
                Ok(source) => source,
                Err(why) => {
                    println!("Err starting source: {:?}", why);

                    msg.reply(ctx, "Error sourcing ffmpeg")
                        .await
                        .expect("error sending message");

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
        }
    //if not in a voice channel
    } else {
        msg.reply(ctx, "Must be in a voice channel to use that command!")
            .await
            .expect("failed to send message");
    }
    Ok(())
}
