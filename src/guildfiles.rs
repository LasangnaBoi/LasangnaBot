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
use songbird::{input::ytdl_search, create_player};
use rand::Rng;

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

///display list of favorites
pub async fn favs(ctx: &Context, msg: &Message) -> Result<()> {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let mut url = String::from("https://upload.wikimedia.org/wikipedia/commons/thumb/4/46/Question_mark_%28black%29.svg/800px-Question_mark_%28black%29.svg.png");
    if let Some(icon) = guild.icon_url() {
        url = icon;
    }
    //let image = guild.icon_url().unwrap();
    let guild_name = guild.name;
    let favpath = format!("./guild_files/{}", guild_id);
    let size: i32 =  read_dir(&favpath).unwrap().count().try_into().expect("failed to parse");

    if size == 0 {
        msg.reply(ctx, "No songs are saved to favorites!".to_string())
            .await
            .expect("guild data has not been initialized");
        return Ok(());
    }

    //embed
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        let i = size;
        //color
        let colour = Colour::from_rgb(149, 8, 2);
        assert_eq!(colour.r(), 149);
        assert_eq!(colour.g(), 8);
        assert_eq!(colour.b(), 2);
        assert_eq!(colour.tuple(), (149, 8, 2));
        m.embed(|e| {
            e.title(format!("{} favorites", guild_name));
            e.thumbnail(url);
            e.description(format!("{} songs in favorites", size));
            e.color(colour);
            for i in 0..i {
                //iterate through info for guild files
                let song = read_dir(&favpath).expect("failed to get path")
                    .nth(i.try_into().expect("failed to parse"));
                //open the file and create an embed field
                let path = song.unwrap().unwrap().path().to_str().unwrap().to_string();
                if let Ok(mut lines) = read_lines(format!("{}/data.txt", &path)) {
                    let title = &lines.next().expect("failed to read line").expect("failed to read line");
                    let channel = &lines.nth(3).expect("failed to read line").expect("failed to read line");
                    e.field(format!("{}. {}", i+1, title), channel, false);
                }
            }
            e
        })
    }).await;
    Ok(())
}

///play song from favorites
pub async fn playfav(ctx: &Context, msg: &Message) -> Result<()> {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let favpath = format!("./guild_files/{}", guild_id);
    let size: i32 =  read_dir(&favpath).unwrap().count().try_into().expect("failed to parse");
    let query: i32 = msg.content[9..].chars().as_str().parse::<i32>().expect("error parsing");
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
            let url = &lines.nth(1).expect("failed to read line").expect("failes to read line");
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

///play random fav
pub async fn randfav(ctx: &Context, msg: &Message) -> Result<()> {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let favpath = format!("./guild_files/{}", guild_id);

    //get song to play
    let size: i32 =  read_dir(&favpath).unwrap().count().try_into().expect("failed to parse");
    if size == 0 {
        msg.reply(ctx, "No songs are saved to favorites!".to_string())
            .await
            .expect("guild data has not been initialized");
        return Ok(());
    }
    let rng = rand::thread_rng().gen_range(1..size+1);
    //create manager
    let manager = songbird::get(ctx)
        .await
        .expect("songbird error")
        .clone();

    //create audio source
    if let Some(handler_lock) = manager.get(guild_id) {
        let mut handler = handler_lock.lock().await;

        let song = read_dir(&favpath).expect("failed to get path")
            .nth((rng-1).try_into().unwrap());

        let path = song.unwrap().unwrap().path().to_str().unwrap().to_string();
        if let Ok(mut lines) = read_lines(format!("{}/data.txt", &path)) {

            //get source from YouTube
            let url = &lines.nth(1).expect("failed to read line").expect("failes to read line");
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

///read lines from a file
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
