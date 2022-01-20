/*
 * guildfiles.rs
 * LasangnaBoi 2022
 * file functionality
 */

use std::fs::*;
use std::io::*;
use std::path::Path;
use serenity::client::Context;
use serenity::utils::Colour;
use serenity::model::channel::Message;

///add current song's data to the guild file
pub async fn addfav(ctx: &Context, msg: &Message) -> std::io::Result<()> {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let path = format!("guild_files/{}", guild_id);
    let dir = format!("./{}", &path);

    //check if the guild is initialized. If it's not, initialize guild_id
    if Path::new(&dir).is_dir() {
    } else {
        create_dir(&dir).expect("failed to guild create directory");
    }

    //create manager
    let manager = songbird::get(ctx)
        .await
        .expect("songbird error")
        .clone();

    //get the queue
    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();
        //check if anything is playing
        if queue.is_empty() {
            msg.reply(ctx, "Nothing is being played!".to_string())
                .await
                .expect("unable to send");
            return Ok(());
        //if something is playing
        } else {
            //get song data
            let song = &queue.current().unwrap().metadata().clone();
            let title = song.title.as_ref().unwrap();
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
            
            //check for song path, if it doesn't exist create it
            let songpath = format!("./{}/{}", &path, &title);
            if Path::new(&songpath).is_dir() {
                msg.reply(ctx, "this song is already in favorites!")
                    .await
                    .expect("error sending message");
            } else {
                let _ = create_dir(&songpath)
                    .expect("error adding song to favorites");
                let datapath = format!("{}/data.txt", &songpath);
                write_songdata(queue, &datapath)
                    .await
                    .expect("unable to write to file");
                let _ = msg.channel_id.send_message(&ctx.http, |m| {
                    m.content("Added to favorites:");
                    m.embed(|e| {
                        e.title(title);
                        e.colour(colour);
                        e.description(channel);
                        e.field("duration: ", duration, false);
                        e.thumbnail(thumbnail);
                        e.url(url);
                        e
                    });
                    m
                }).await;
            }
        }
    }
    Ok(())
}

///write song data to a file
async fn write_songdata(queue: &songbird::tracks::TrackQueue, path: &str) -> Result<()> {
    let song = queue.current().unwrap().metadata().clone();
    let title = song.title.as_ref().unwrap();
    let url = song.source_url.as_ref().unwrap();
    let thumbnail = song.thumbnail.as_ref().unwrap();
    let duration = song.duration.as_ref().unwrap();
    let channel = song.channel.as_ref().unwrap();
    //open the file
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .expect("unable to open file");
    let mut f = LineWriter::new(f);
    //write to the file
    f.write_all(format!("{}\n", title).as_bytes()).expect("unable to write to file");
    f.flush().expect("unable to flush");
    f.write_all(format!("{}\n", url).as_bytes()).expect("unable to write to file");
    f.flush().expect("unable to flush");
    f.write_all(format!("{}\n", thumbnail).as_bytes()).expect("unable to write to file");
    f.flush().expect("unable to flush");
    f.write_all(format!("{}\n", duration.as_secs()).as_bytes()).expect("unable to write to file");
    f.flush().expect("unable to flush");
    f.write_all(format!("{}\n", channel).as_bytes()).expect("unable to write to file");
    f.flush().expect("unable to flush");
    Ok(())
}
