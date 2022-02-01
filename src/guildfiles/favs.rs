/*
 * favs.rs
 * LasangnaBoi 2022
 * display the server's favorited songs
 */

use crate::*;
use guildfiles::*;

pub async fn favs(ctx: &Context, msg: &Message) -> Result<()> {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let mut url = String::from("https://upload.wikimedia.org/wikipedia/commons/thumb/4/46/Question_mark_%28black%29.svg/800px-Question_mark_%28black%29.svg.png");
    if let Some(icon) = guild.icon_url() {
        url = icon;
    }
    let image = guild.icon_url().unwrap();
    let guild_name = guild.name;
    let favpath = format!("./guild_files/{}", guild_id);
    let size: i32 =  read_dir(&favpath).unwrap().count().try_into().expect("failed to parse");

    if size == 0 {
        msg.reply(ctx, "No songs are saved to favorites!".to_string())
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
    let songs = getsongs(&favpath, &size);
    let pages = size/10;
    let mut remsongs = songs.clone();
    let mut n = 1;
    let mut embeds: Vec<CreateEmbed> = Vec::new();
    for _ in 0..pages {
        let binding = remsongs.clone();
        let (current, remaining) = binding.split_at(10);
        let mut e = CreateEmbed::default();
        e.title(format!("{} favorites", guild_name));
        e.thumbnail(image.clone());
        e.description(format!("{} songs in favorites", size));
        e.color(colour);
        for i in current.iter() {
            let title = i.title.clone();
            let channel = i.channel.clone(); 
            e.field(format!("{}. {}", n, title), channel, false);
            n += 1;
        }
        remsongs = remaining.to_vec();
        embeds.push(e);
    }
    if !remsongs.is_empty() {
        let mut e = CreateEmbed::default();
        e.title(format!("{} favorites", guild_name));
        e.thumbnail(image.clone());
        e.description(format!("{} songs in favorites", size));
        e.color(colour);
        for i in remsongs.iter() {
            let title = i.title.clone();
            let channel = i.channel.clone(); 
            e.field(format!("{}. {}", n, title), channel, false);
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
    }).await.expect("failed to send message");

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
            .await.expect("failed to get iteraction");
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
            .await.expect("failed to get iteraction");
        }
    }
    Ok(())
}
