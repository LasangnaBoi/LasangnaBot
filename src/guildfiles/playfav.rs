/*
 * playfav.rs
 * LasangnaBoi 2022
 * play a song from favorites using a menu
 */

use crate::*;
use guildfiles::*;

///play song from favorites
pub async fn playfav(ctx: &Context, msg: &Message) -> Result<()> {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let favpath = format!("./guild_files/{}", guild_id);
    let size: i32 =  read_dir(&favpath).unwrap().count().try_into().expect("failed to parse");
    if size == 0 {
        msg.reply(ctx, "No songs are saved to favorites!".to_string())
            .await
            .expect("guild data has not been initialized");
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

        //create song menu
        let songs = getsongs(&favpath, &size);
        let mut menu = CreateSelectMenu::default();
        menu.min_values(1);
        menu.max_values(1);
        menu.custom_id("favmenu");
        menu.placeholder("select song");
        menu.options(|f| {
            for i in 0..size {
                let song = songs.clone().into_iter().nth(i.try_into().unwrap()).unwrap();
                let mut opt = CreateSelectMenuOption::default();
                opt.label(song.title.to_string());
                opt.value(song.url.to_string());
                f.add_option(opt);
            }
            f
        });
        //create button
        let mut last = CreateButton::default();
        last.custom_id("play");
        last.label("play");
        last.style(ButtonStyle::Success);

        let mut ar = CreateActionRow::default();
        ar.add_select_menu(menu);

        let m = msg.channel_id.send_message(&ctx.http, |m| {
            m.content("Select song from favorites: ");
            m.components(|c| c.add_action_row(ar))
        }).await.expect("failed to create message");

        let mut url = String::from("url");

        let mci =
            match m.await_component_interaction(&ctx).timeout(Duration::from_secs(60 * 3)).await {
                Some(ci) => ci,
                None => {
                    m.reply(&ctx, "Timed out").await.unwrap();
                    return Ok(());
                },
            };
        mci.create_interaction_response(&ctx, |r| {
            r.kind(InteractionResponseType::UpdateMessage);
            r.interaction_response_data(|d| {
                url = mci.data.values.clone().into_iter().next().unwrap();
                d.content("playing selection...")
            })
        })
        .await
        .unwrap();

        //get source from YouTube
        let source = match ytdl_search(url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);
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

    //if not in a voice channel
    } else {
        msg.reply(ctx, "Must be in a voice channel to use that command!")
            .await
            .expect("failed to send message");
    }
    Ok(())
}
