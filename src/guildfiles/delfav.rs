/*
 * delfav.rs
 * LasangnaBoi 2022
 * delete a song from favorites
 */

use crate::*;
use guildfiles::*;
use tokio::fs::remove_dir_all;

pub async fn delfav(ctx: &Context, msg: &Message) -> Result<()> {
    //get guild
    let guild = match msg.guild(&ctx.cache).await {
        Some(guild) => guild,
        None => {
            msg.reply(ctx, "unable to find guild").await.expect("failed to send message");
            return Ok(());
        } 
    };
    let guild_id = guild.id;
    //get the favorites path
    let favpath = format!("guild_files/{}", guild_id);
    let size =  read_dir(&favpath)?.count();
    if size == 0 {
        msg.reply(ctx, "No songs are saved to favorites!".to_string())
            .await.expect("failed to send message");
        return Ok(());
    }
    //check for a valid index
    let query: usize = match msg.content[8..].chars().as_str().parse::<usize>() {
        Ok(query) => query,
        Err(_) => {
            msg.reply(ctx, "not a valid index!").await.expect("failed to send message");
            return Ok(());
        }
    };
    if query>0 {
        //get favorites directory
        let mut favdir = read_dir(&favpath)?;
        let _ = match favdir.nth(query-1) {
            Some(songdir) => {
                remove_dir_all(songdir?.path()).await?;
                msg.reply(ctx, "removed song from favorites").await.expect("failed to send message");
            },
            None => {
                msg.reply(ctx, "not a valid index!").await.expect("failed to send message");
                return Ok(());
            },
        };
    } else {
        msg.reply(ctx, "not a valid index!").await.expect("failed to send message");
    }
    Ok(())
}
