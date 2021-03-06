/*
 * LasangnaBot
 * LasangnaBoi 2022
 * a discord bot made in Rust
 */
#![feature(path_try_exists)]
mod guildfiles;
mod images;
mod misc;
mod voice;

use dotenv::dotenv;
use serenity::{
    async_trait,
    client::{Client, Context, EventHandler},
    framework::{
        standard::{
            macros::{command, group},
            Args, CommandResult,
        },
        StandardFramework,
    },
    model::{
        channel::Message,
        gateway::{Activity, Ready},
    },
    utils::Colour,
    Result as SerenityResult,
};
use songbird::SerenityInit;
use std::env;
use tokio::join;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    ///on connect
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        let t1 = ctx.set_activity(Activity::listening("my parents fight"));
        join!(t1);
    }
    ///when a message is sent
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.to_lowercase().contains("uwu") {
            images::e621(&ctx, &msg).await.expect("failed to send message");
        }
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect("create .env file in project root");
    tracing_subscriber::fmt::init();

    let token = env::var("DISCORD_TOKEN").expect("expected discord token in .env file");
    let prefix = env::var("PREFIX").expect("expected prefix in .env file");
    let application_id = env::var("APPLICATION_ID").expect("expected application id in .env file");

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(&prefix))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .application_id(application_id.parse().expect("failed to parse"))
        .framework(framework)
        .register_songbird()
        .await
        .expect("Error creating client");
    tokio::spawn(async move {
        let _ = client
            .start()
            .await
            .map_err(|why| println!("Client ended: {:?}", why));
    });

    tokio::signal::ctrl_c().await.expect("not a command");
    println!("Received Ctrl-C, shutting down.");
}

#[group]
#[commands(
    ping, join, leave, play, skip, stop, playing, queue, addfav, favs, playfav, randfav, playfavat,
    help, dad, delfav
)]
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
    voice::join::join(ctx, msg).await.expect("error joining");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn leave(ctx: &Context, msg: &Message) -> CommandResult {
    voice::leave::leave(ctx, msg)
        .await.expect("error leaving channel");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn play(ctx: &Context, msg: &Message) -> CommandResult {
    voice::play::play(ctx, msg)
        .await.expect("error finding song");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn skip(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    voice::skip::skip(ctx, msg, _args)
        .await.expect("error skipping song");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn stop(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    voice::stop::stop(ctx, msg, _args)
        .await.expect("error stopping");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn playing(ctx: &Context, msg: &Message) -> CommandResult {
    voice::playing::playing(ctx, msg)
        .await.expect("error getting current song");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    voice::queue::queue(ctx, msg)
        .await.expect("error getting queue");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn addfav(ctx: &Context, msg: &Message) -> CommandResult {
    guildfiles::addfav::addfav(ctx, msg)
        .await.expect("unable to write file");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn favs(ctx: &Context, msg: &Message) -> CommandResult {
    guildfiles::favs::favs(ctx, msg)
        .await.expect("unable to retrieve guild files");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn playfav(ctx: &Context, msg: &Message) -> CommandResult {
    guildfiles::playfav::playfav(ctx, msg)
        .await.expect("unable to retrieve guild files");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn randfav(ctx: &Context, msg: &Message) -> CommandResult {
    guildfiles::randfav::randfav(ctx, msg)
        .await.expect("unable to retrieve guild files");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn playfavat(ctx: &Context, msg: &Message) -> CommandResult {
    guildfiles::playfavat::playfavat(ctx, msg)
        .await.expect("unable to retrieve guild files");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn help(ctx: &Context, msg: &Message) -> CommandResult {
    misc::help::help(ctx, msg).await.expect("command failed");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn dad(ctx: &Context, msg: &Message) -> CommandResult {
    misc::dad::dad(ctx, msg).await.expect("command failed");
    Ok(())
}

#[command]
#[only_in(guilds)]
async fn delfav(ctx: &Context, msg: &Message) -> CommandResult {
    guildfiles::delfav::delfav(ctx, msg).await.expect("command failed");
    Ok(())
}

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
