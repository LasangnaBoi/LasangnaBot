/*
 * voice.rs
 * LasangnaBoi 2022
 * voice channel functionality
 */

pub mod join;
pub mod leave;
pub mod play;
pub mod playing;
pub mod queue;
pub mod skip;
pub mod stop;

use serenity::{
    client::Context,
    framework::
        standard::{
            Args,
            CommandResult,
        },
    model::{channel::Message, misc::Mentionable},
};
use songbird::{input::ytdl_search, create_player};
use serenity::utils::Colour;
