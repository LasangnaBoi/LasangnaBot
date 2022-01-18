/*
 * voice.rs, LsangnaBoi 2022
 * voice channel functionality
 */

use crate::check_msg;
use serenity::{
    client::Context,
    framework::
        standard::{
            Args,
            CommandResult,
        },
    model::{channel::Message, misc::Mentionable},
};
use songbird::{input::ytdl_search, create_player};
use serenity::utils::Colour;

///join voice channel
pub async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let channel_id = guild
        .voice_states
        .get(&msg.author.id)
        .and_then(|voice_state| voice_state.channel_id);

    let connect_to = match channel_id {
        Some(channel) => channel,
        None => {
            check_msg(msg.reply(ctx, "Not in a voice channel").await);

            return Ok(());
        },
    };

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let (_handle_lock, success) = manager.join(guild_id, connect_to).await;

    if let Ok(_channel) = success {
        check_msg(
            msg.channel_id
                .say(&ctx.http, &format!("Joined {}", connect_to.mention()))
                .await,
        );

    } else {
        check_msg(
            msg.channel_id
                .say(&ctx.http, "Error joining the channel")
                .await,
        );
    }
    Ok(())
}

///leave voice channel
pub async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;

    let manager = songbird::get(ctx)
        .await
        .expect("Songbird Voice client placed in at initialisation.")
        .clone();

    let has_handler = manager.get(guild_id).is_some();

    if has_handler {
        if let Err(e) = manager.remove(guild_id).await {
            check_msg(
                msg.channel_id
                    .say(&ctx.http, format!("Failed: {:?}", e))
                    .await,
            );
        }
        check_msg(msg.channel_id.say(&ctx.http, "Left voice channel").await);

    } else {
        check_msg(msg.reply(ctx, "Not in a voice channel").await);
    }
    Ok(())
}

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
            });
            m
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
        let queue = handler.queue();

        if queue.is_empty() {
            check_msg(msg.channel_id.say(&ctx.http, format!("The queue is empty!")).await);
            return Ok(());

        } else {
            let _ = queue.skip();

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
        let queue = handler.queue();
        let _ = queue.stop();
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
        let queue = handler.queue();

        if queue.is_empty() {
            check_msg(msg.channel_id.say(&ctx.http, format!("Nothing is being played!")).await);
            return Ok(());

        } else {
            let song = &queue.current().unwrap().metadata().clone();

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
                });
                m
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
        let queue = handler.queue();

        if queue.is_empty() {
            check_msg(msg.channel_id.say(&ctx.http, format!("The queue is empty!")).await);
            return Ok(());
        }

        //create embed
        let _ = msg.channel_id.send_message(&ctx.http, |m| {
            let i: usize;
            if queue.len() < 10 {
                i = queue.len();
            } else {
                i = 10;
            }
            //color
            let colour = Colour::from_rgb(149, 8, 2);
            assert_eq!(colour.r(), 149);
            assert_eq!(colour.g(), 8);
            assert_eq!(colour.b(), 2);
            assert_eq!(colour.tuple(), (149, 8, 2));
            m.content("Current queue:");
            m.embed(|e| {
                e.title(format!("current length: {}", queue.len()));
                e.color(colour);
                for i in 0..i {
                    let song = &queue.current_queue().get(i).unwrap().metadata().clone();
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
            });
            m
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
