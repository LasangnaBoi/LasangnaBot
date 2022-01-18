/*
 * LasngnaBot
 * LasangnaBoi 2022
 * a discord bot made in Rust
 */

mod voice;

use tokio::join;
use songbird::SerenityInit;
use serenity::client::Context;
use serenity::{
    async_trait,
    client::{Client, EventHandler},
    framework::{
        StandardFramework,
        standard::{
            Args, CommandResult,
            macros::{command, group},
        },
    },
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
    tracing_subscriber::fmt::init();

    let token = "ODkwNDUxNTI1Mzc2MjgyNjI0.YUv_mw.aw5eh6xtouztlY2E9WGJGadkC8E";

    let framework = StandardFramework::new()
        .configure(|c| c
                   .prefix("$"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&token)
        .event_handler(Handler)
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
#[commands(ping, join, leave, play, skip, stop, playing, queue)]
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

/// Checks that a message successfully sent; if not, then logs why to stdout.
fn check_msg(result: SerenityResult<Message>) {
    if let Err(why) = result {
        println!("Error sending message: {:?}", why);
    }
}
