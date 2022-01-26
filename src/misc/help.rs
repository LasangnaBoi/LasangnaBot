/*
 * help.rs
 * LasangnaBoi 2022
 * displays all the bot's commands and their use
 */

use crate::*;

pub async fn help(ctx: &Context, msg: &Message) -> CommandResult {

    //create voice help embed
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

    //create favorites help embed
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
            e.field("addfav", "remove a song from favorites", false);
            e.field("favs", "List the server's favorited songs.", false);
            e.field("playfav", "Creates a dropdown menu of favorites, select a song to play it. Must me in a voice channel to use.", false);
            e.field("playfavat", "Play a song, using the index of the song from the 'favs' command as an argument. For example, '$playfavat 4' will play the fourth song on the favorites list. Must be in a voice channel to use.", false);
            e.field("randfav", "Play a random song from the server's favorites, must be in a voice channel to use.", false);
            e
        })
    }).await;

    Ok(())
}
