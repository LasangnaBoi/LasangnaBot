/*
 * LasangnaBot
 * LasangnaBoi 2022
 * a discord bot made in Rust
 */
#![feature(path_try_exists)]
mod voice;
mod guildfiles;

use dotenv::dotenv;
use std::env;
use tokio::join;
use songbird::SerenityInit;
use serenity::{
    async_trait,
    client::{Client, EventHandler, Context},
    framework::{
        StandardFramework,
        standard::{
            Args, CommandResult,
            macros::{command, group},
        },
    },
    utils::Colour,
    model::{channel::Message, gateway::{Ready, Activity}},
    Result as SerenityResult,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    /// On connect
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let t1 = ctx.set_activity(Activity::listening("my parents fight"));
        join!(t1);
    }
}

#[tokio::main]
async fn main()
{
    dotenv().expect("create .env file in project root");
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("expected discord token in .env file");
    let prefix = env::var("PREFIX").expect("expected prefix in .env file");
    let application_id = env::var("APPLICATION_ID").expect("expected application id in .env file");

    let framework = StandardFramework::new()
        .configure(|c| c
                   .prefix(&prefix))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .application_id(application_id.parse().expect("failed to parse"))
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");
    tokio::spawn(async move {
        let _ = client.start().await.map_err(|why| println!("Client ended: {:?}", why));
    });

    tokio::signal::ctrl_c().await
    .expect("not a command");
    println!("Received Ctrl-C, shutting down.");
}

#[group]
#[commands(ping, join, leave, play, skip, stop, playing, queue, addfav, favs, playfav, randfav, playfavat, help)]
pub struct General;

#[command]
#[only_in(guilds)]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    check_msg(msg.channel_id.say(&ctx.http, "pong!").await);
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn join(ctx: &Context, msg: &Message) -> CommandResult {
    voice::join(ctx, msg).await.expect("error joining");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    voice::leave(ctx, msg).await.expect("error leaving channel");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    voice::play(ctx, msg).await.expect("error finding song");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    voice::skip(ctx, msg, _args).await.expect("error skipping song");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    voice::stop(ctx, msg, _args).await.expect("error stopping");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn playing(ctx: &Context, msg: &Message) -> CommandResult {
    voice::playing(ctx, msg).await.expect("error getting current song");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    voice::queue(ctx, msg).await.expect("error getting queue");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn addfav(ctx: &Context, msg: &Message) -> CommandResult {
    guildfiles::addfav(ctx, msg).await.expect("unable to write file");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn favs(ctx: &Context, msg: &Message) -> CommandResult {
    guildfiles::favs(ctx, msg).await.expect("unable to retrieve guild files");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn playfav(ctx: &Context, msg: &Message) -> CommandResult {
    guildfiles::playfav(ctx, msg).await.expect("unable to retrieve guild files");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn randfav(ctx: &Context, msg: &Message) -> CommandResult {
    guildfiles::randfav(ctx, msg).await.expect("unable to retrieve guild files");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn playfavat(ctx: &Context, msg: &Message) -> CommandResult {
    guildfiles::playfavat(ctx, msg).await.expect("unable to retrieve guild files");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
//#[commands(ping, join, leave, play, skip, stop, playing, queue, addfav, favs, playfav, randfav, playfavat, help)]
    //create embeds
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        //color
        let colour = Colour::from_rgb(149, 8, 2);
        assert_eq!(colour.r(), 149);
        assert_eq!(colour.g(), 8);
        assert_eq!(colour.b(), 2);
        assert_eq!(colour.tuple(), (149, 8, 2));
        m.embed(|e| {
            e.title("Voice Commands");
            e.color(colour);
            e.field("join", "Join voice channel. Required to use voice functionality, must be in a voice channel to use.", false);
            e.field("leave", "Disconnect the bot from the current voice channel, must be in a voice channel to use.", false);
            e.field("play", "Search YouTube for a song using a provided argument. For example. '$play a song' will search youtube for 'a song'. Must be in a voice channel to use.", false);
            e.field("skip", "Skip the current song, must be in a voice channel to use.", false);
            e.field("stop", "Stop playing and clear the queue, must be in a voice channel to use", false);
            e.field("playing", "Get information from current song, must be in a voice channel to use.", false);
            e.field("queue", "Get the current queue.", false);
            e
        })
    }).await;
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        //color
        let colour = Colour::from_rgb(149, 8, 2);
        assert_eq!(colour.r(), 149);
        assert_eq!(colour.g(), 8);
        assert_eq!(colour.b(), 2);
        assert_eq!(colour.tuple(), (149, 8, 2));
        m.embed(|e| {
            e.title("Favorites Commands");
            e.color(colour);
            e.field("addfav", "Add the current song to favorites, must be in a voice channel to use.", false);
            e.field("favs", "List the server's favorited songs.", false);
            e.field("playfav", "Creates a dropdown menu of favorites, select a song to play it. Must me in a voice channel to use.", false);
            e.field("playfavat", "Play a song, using the index of the song from the 'favs' command as an argument. For example, '$playfavat 4' will play the fourth song on the favorites list. Must be in a voice channel to use.", false);
            e.field("randfav", "Play a random song from the server's favorites, must be in a voice channel to use.", false);
            e
        })
    }).await;

    Ok(())
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
