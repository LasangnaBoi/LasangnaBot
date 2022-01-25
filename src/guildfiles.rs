/*
 * guildfiles.rs
 * LasangnaBoi 2022
 * file functionality
 */

pub mod addfav;
pub mod favs;
pub mod playfav;
pub mod playfavat;
pub mod randfav;
pub mod delfav;

use rand::Rng;
use std::{vec::*, fs::*, io::*, path::Path, time::Duration};
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
        CreateSelectMenuOption,
        CreateSelectMenu
    },
    futures::StreamExt};
use songbird::{
    create_player,
    input::ytdl_search,
};

//song struct
#[allow(dead_code)]
#[derive(Clone)]
struct Song {
    title: String,
    url: String,
    duration: String,
    thumbnail: String,
    channel: String,
}

//get a vector of songs
fn getsongs(path: &str, size: &i32) -> Vec<Song> {
    let mut songs: Vec::<Song> = Vec::new();
    for i in 0..*size  {
        let song = read_dir(&path).expect("failed to get path")
            .nth(i.try_into().expect("failed to parse"));
        let path = song.unwrap().unwrap().path().to_str().unwrap().to_string();
        if let Ok(mut lines) = read_lines(format!("{}/data.txt", &path)) {
            let title = &lines.next().expect("failed to read line").expect("failed to read line");
            let url = &lines.next().expect("failed to read line").expect("failed to read line");
            let thumbnail = &lines.next().expect("failed to read line").expect("failed to read line");
            let duration = &lines.next().expect("failed to read line").expect("failed to read line");
            let channel = &lines.next().expect("failed to read line").expect("failed to read line");
            let song = Song {
                title: title.to_string(),
                url: url.to_string(),
                duration: duration.to_string(),
                thumbnail: thumbnail.to_string(),
                channel: channel.to_string(),
            };
            songs.push(song);
        }
    }
    songs
}

///write song data to a file
async fn write_songdata(queue: &songbird::tracks::TrackQueue, path: &str) -> Result<()> {
    let song = queue.current().unwrap().metadata().clone();
    let title = song.title.as_ref().unwrap();
    let url = song.source_url.as_ref().unwrap();
    let thumbnail = song.thumbnail.as_ref().unwrap();
    let duration = song.duration.as_ref().unwrap();
    let channel = song.channel.as_ref().unwrap();
    //open the file
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .append(true)
        .open(path)
        .expect("unable to open file");
    let mut f = LineWriter::new(f);
    //write to the file
    f.write_all(format!("{}\n", title).as_bytes()).expect("unable to write to file");
    f.flush().expect("unable to flush");
    f.write_all(format!("{}\n", url).as_bytes()).expect("unable to write to file");
    f.flush().expect("unable to flush");
    f.write_all(format!("{}\n", thumbnail).as_bytes()).expect("unable to write to file");
    f.flush().expect("unable to flush");
    f.write_all(format!("{}\n", duration.as_secs()).as_bytes()).expect("unable to write to file");
    f.flush().expect("unable to flush");
    f.write_all(format!("{}\n", channel).as_bytes()).expect("unable to write to file");
    f.flush().expect("unable to flush");
    Ok(())
}

///read lines from a file
pub fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}
