/*
 * guildfiles.rs
 * LasangnaBoi
 * file functionality
 */

use std::io::prelude::*;
use std::fs::*;
use std::path::Path;
use serenity::client::Context;
use serenity::model::channel::Message;

///check for guild files
pub async fn init_guild(ctx: &Context, msg: &Message) -> std::io::Result<()> {
    let guild = msg.guild(&ctx.cache).await.unwrap().id;
    let path = format!("guild_files/{}.txt", guild);
    if Path::new(&path).is_file() {
        println!("file already exists!");
        Ok(())
    } else {
        let mut file = File::create(&path)?;
        println!("new file added: {}", path);
        file.write_all(b"wow, you wrote to a file!")?;
        Ok(())
    }
}
