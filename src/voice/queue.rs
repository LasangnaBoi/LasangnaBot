/*
 * queue.rs
 * LasangnaBoi 2022
 * get the current queue
 */

use crate::*;
use std::time::Duration;
use serenity::{
    client::Context,
    utils::Colour,
    model::{
        channel::Message,
        interactions::{
            message_component::ButtonStyle, InteractionResponseType,
        }
    },
    builder::{
        CreateEmbed,
        CreateButton,
        CreateActionRow, 
    },
    futures::StreamExt};

#[derive(Clone)]
struct Song {
    title: String,
    url: String,
    channel: String,
    duration: String,
}

async fn getsongs(ctx: &Context, msg: &Message) -> Vec<Song> {
    let mut songs: Vec::<Song> = Vec::new();
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    //create manager
    let manager = songbird::get(ctx)
        .await
        .expect("songbird error")
        .clone();
    if let Some(handler_lock) = manager.get(guild_id) {
        let handler = handler_lock.lock().await;
        let queue = handler.queue();

        for i in queue.current_queue() {
            let song = Song {
                title: i.metadata().title.as_ref().unwrap().to_string(),
                url: i.metadata().source_url.as_ref().unwrap().to_string(),
                channel: i.metadata().channel.as_ref().unwrap().to_string(),
                duration: {
                    let time = i.metadata().duration.as_ref().unwrap();
                    let minutes = time.as_secs()/60;
                    let seconds = time.as_secs()-minutes*60;
                    format!("{}:{:02}", minutes, seconds)
                }
            };
            songs.push(song);
        }
    }
    songs
}

///get the queue
pub async fn queue(ctx: &Context, msg: &Message) -> CommandResult {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let image = guild.icon_url().unwrap();
    let songs = getsongs(ctx, msg).await;
    let size = songs.len();
    if size == 0 {
        msg.reply(ctx, "The queue is empty!".to_string())
            .await
            .expect("guild data has not been initialized");
        return Ok(());
    }

    //color
    let colour = Colour::from_rgb(149, 8, 2);
    assert_eq!(colour.r(), 149);
    assert_eq!(colour.g(), 8);
    assert_eq!(colour.b(), 2);
    assert_eq!(colour.tuple(), (149, 8, 2));
    let pages = size/10;
    let mut remsongs = songs.clone();
    let mut n = 1;
    let mut embeds: Vec<CreateEmbed> = Vec::new();
    for _ in 0..pages {
        let binding = remsongs.clone();
        let (current, remaining) = binding.split_at(10);
        let mut e = CreateEmbed::default();
        e.title("current queue:");
        e.thumbnail(image.clone());
        e.description(format!("{} songs in queue", size));
        e.color(colour);
        for i in current.iter() {
            let title = i.title.clone();
            let channel = i.channel.clone(); 
            let duration = i.duration.clone();
            e.field(format!("{}. {} | {}", n, title, channel), duration, false);
            n += 1;
        }
        remsongs = remaining.to_vec();
        embeds.push(e);
    }
    if !remsongs.is_empty() {
        let mut e = CreateEmbed::default();
        e.title("current queue:");
        e.thumbnail(image.clone());
        e.description(format!("{} songs in queue", size));
        e.color(colour);
        for i in remsongs.iter() {
            let title = i.title.clone();
            let channel = i.channel.clone();
            let duration = i.duration.clone();
            e.field(format!("{}. {} | {}", n, title, channel), duration, false);
            n += 1;
        }
        embeds.push(e);
    }

    //create buttons
    let mut last = CreateButton::default();
    last.custom_id("last");
    last.label("last");
    last.style(ButtonStyle::Primary);
    let mut next = CreateButton::default();
    next.custom_id("next");
    next.label("next");
    next.style(ButtonStyle::Primary);

    //create action row
    let mut ar = CreateActionRow::default();
    ar.add_button(last);
    ar.add_button(next);

    let mut i = 10;
    if size <= 10 {
        i = size;
    }

    //create message
    let m = msg.channel_id.send_message(&ctx.http, |m| {
        m.components(|c| c.add_action_row(ar));
        m.embed(|e| {
            e.title("current queue:");
            e.thumbnail(image.clone());
            e.description(format!("{} songs in queue", size));
            e.color(colour);
            for i in 0..i {
                let song = songs.clone().into_iter().nth(i.try_into().unwrap()).unwrap();
                let title = song.title.clone();
                let channel = song.channel.clone(); 
                let duration = song.duration.clone();
                e.field(format!("{}. {} | {}", i+1, title, channel), duration, false);
            }
            e
        })
    }).await.unwrap();

    //create interaction response
    let mut n = 0;
    let mut cib = m.await_component_interactions(&ctx).timeout(Duration::from_secs(60*3)).await;
    while let Some(mci) = cib.next().await {
        if mci.data.custom_id == "last" {
            mci.create_interaction_response(&ctx, |r| {
                r.kind(InteractionResponseType::UpdateMessage);
                r.interaction_response_data(|d| {
                    if n != 0 {
                        n -= 1;
                        d.add_embed(embeds.clone().into_iter().nth(n).unwrap())
                    } else {
                        d.add_embed(embeds.clone().into_iter().next().unwrap())
                    }
                })
            })
            .await
            .unwrap();
        }
        if mci.data.custom_id == "next" {
            mci.create_interaction_response(&ctx, |r| {
                r.kind(InteractionResponseType::UpdateMessage);
                r.interaction_response_data(|d| {
                    if n < pages.try_into().unwrap() {
                        n += 1;
                        d.add_embed(embeds.clone().into_iter().nth(n).unwrap())
                    } else {
                        d.add_embed(embeds.clone().into_iter().nth(pages.try_into().expect("failed to parse")).unwrap())
                    }
                })
            })
            .await
            .unwrap();
        }
    }
    Ok(())
}









