/*
 * images
 * LasangnaBoi 2022
 * for use against really annoying people
 */

use crate::*;
use rs621::client::Client;
use serenity::{
    client::Context,
    model::channel::Message,
    futures::StreamExt};
use rand::Rng;

pub async fn e621(ctx: &Context, msg: &Message) -> CommandResult {
    let client = Client::new("https://e621.net", "LasangnaBot/1.0 (by LasangnaBot on e621)")?;
    let mut results = client
        .post_search(&["-flash", "-webm", "-child", "rating:s"][..])
        .take(300);
    let mut urls: Vec<String> = Vec::new();
    while let Some(post) = results.next().await {
        match post {
            Ok(post) => {
                let url: url::Url = post.file.url.unwrap().parse().unwrap();
                urls.push(url.to_string());
            }
            Err(_) => {
                msg.reply(ctx, "unable to source image")
                    .await.expect("failed to send message"); 
            }
        }
    }
    //create message
    let rng = rand::thread_rng().gen_range(0..299);
    let url = urls.into_iter().nth(rng).unwrap().to_string();
    let tag = msg.author.tag();
    println!("{}", url);
    let _ = msg.channel_id.send_message(&ctx.http, |m| {
        m.embed(|e| {
            e.title(format!("shut the fuck up {}", tag));
            e.image(url);
            e
        })
    }).await.expect("unable to send message");
    Ok(())
}
