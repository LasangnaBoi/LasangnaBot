/*
 * delfav.rs
 * LasangnaBoi 2022
 * delete a song from favorites
 */

use crate::*;
use guildfiles::*;

pub async fn delfav(ctx: &Context, msg: &Message) -> Result<()> {
    let guild = msg.guild(&ctx.cache).await.unwrap();
    let guild_id = guild.id;
    let favpath = format!("guild_files/{}", guild_id);
    let size: i32 =  read_dir(&favpath).unwrap().count().try_into().expect("failed to parse");
    if size == 0 {
        msg.reply(ctx, "No songs are saved to favorites!".to_string())
            .await
            .expect("guild data has not been initialized");
        return Ok(());
    }
    let query: usize = msg.content[8..].chars().as_str().parse::<usize>().expect("error parsing");
    if size == 0 {
        msg.reply(ctx, "not a valid entry!")
            .await
            .expect("guild data has not been initialized");
        return Ok(());
    }
    //get favorites directory
    let favdir = read_dir(&favpath);
    match favdir {
        Ok(mut favdir) => {
            //get song folder
            let songdir = favdir.nth(query-1);
            match songdir {
                //get path
                Some(path) => {
                    match path {
                        Ok(entry) => {
                            //remove directory
                            match remove_dir_all(entry.path()) {
                                Ok(()) => {
                                    msg.reply(&ctx.http, "song removed from favorites")
                                        .await.expect("failed to send message");
                                }
                                Err(_) => {
                                    msg.reply(&ctx.http, "unable to remove song from favorites")
                                        .await.expect("failed to send message");
                                    return Ok(());
                                }
                            }
                        },
                        Err(_) => {
                            msg.reply(&ctx.http, "unable to remove song from favorites")
                                .await.expect("failed to send message");
                            return Ok(());
                        }
                    }
                },
                None => {
                    msg.reply(&ctx.http, "unable to remove song from favorites")
                        .await.expect("failed to send message");
                    return Ok(())
                }
            }
        },
        Err(_) => {
            msg.reply(&ctx.http, "unable to remove song from favorites")
                .await.expect("failed to send message");
            return Ok(());
        }
    }
    Ok(())
}
