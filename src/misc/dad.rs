/*
 * dad.rs
 * LasangnaBot
 * use in response to fatherless behavior
 */

use crate::*;
use guildfiles::read_lines;
use rand::Rng;
use std::vec::Vec;

pub async fn dad(ctx: &Context, msg: &Message) -> CommandResult {
    //get the file of quotes
    let mut quotes: Vec<String> = Vec::new();
    let size: i32 = read_lines("./misc/dadquotes/dadquotes.txt")
        .unwrap().count().try_into().expect("failed to parse");
    //select a random quote
    if let Ok(mut lines) = read_lines("./misc/dadquotes/dadquotes.txt") {
        for _ in 0..size-1 {
            let quote = lines.next()
                .expect("failed to read line")
                .expect("failed to read line");
            quotes.push(quote.to_string());
        }
    }
    let rng = rand::thread_rng().gen_range(0..size-1);
    let quote = quotes.into_iter().nth(rng.try_into().unwrap())
        .expect("failed to get line");
    msg.reply_ping(&ctx, quote).await.expect("failed to send message");
    Ok(())
}
