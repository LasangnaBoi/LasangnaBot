/*
 * LasangnaBot
 * LasangnaBoi 2021
 * a discord bot made in Rust
 */

mod voice;
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        id::GuildId,
        interactions::{
            application_command::{
            ApplicationCommandOptionType, ApplicationCommand},
            Interaction,
            InteractionResponseType,
        },
    },
    prelude::*,
};
use songbird::SerenityInit;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            let content = match command.data.name.as_str() {
                "ping" => "Hey, I'm alive!".to_string(),
                "join" => voice::join(&ctx, &command).await,
                "play" => voice::play(&ctx, &command).await,
                "skip" => voice::skip(&ctx, &command).await,
                "stop" => voice::stop(&ctx, &command).await,
                "playing" => voice::playing(&ctx, &command).await,
                "queue" => voice::queue(&ctx, &command).await,
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let _guild_id = GuildId(yourtestguildidhere);
        /*
        let _commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command
                        .name("join")
                        .description("join voice channel")
                })
                .create_application_command(|command| {
                    command
                        .name("play")
                        .description("search youtube for a song")
                        .create_option(|option| {
                            option
                                .name("query")
                                .description("what to search youtube for")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                            })
                })
                .create_application_command(|command| {
                    command
                        .name("skip")
                        .description("skip the current song")
                })
                .create_application_command(|command| {
                    command
                        .name("stop")
                        .description("stop playing and clear the queue")
                })
                .create_application_command(|command| {
                    command
                        .name("playing")
                        .description("get info for the current song")
                })
                .create_application_command(|command| {
                    command
                        .name("queue")
                        .description("get info for the queue")
                })
        })
        .await;
*/
        let _global_commands = ApplicationCommand::set_global_application_commands(&ctx.http, |commands| {
            commands
                .create_application_command(|command| {
                    command.name("ping").description("A ping command")
                })
                .create_application_command(|command| {
                    command
                        .name("join")
                        .description("join voice channel")
                })
                .create_application_command(|command| {
                    command
                        .name("play")
                        .description("search youtube for a song")
                        .create_option(|option| {
                            option
                                .name("query")
                                .description("what to search youtube for")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                            })
                })
                .create_application_command(|command| {
                    command
                        .name("skip")
                        .description("skip the current song")
                })
                .create_application_command(|command| {
                    command
                        .name("stop")
                        .description("stop playing and clear the queue")
                })
                .create_application_command(|command| {
                    command
                        .name("playing")
                        .description("get info for the current song")
                })
                .create_application_command(|command| {
                    command
                        .name("queue")
                        .description("get info for the queue")
                })
        })

            .await;
    }
}

#[tokio::main]
async fn main() {
    // Configure the client with your Discord bot token in the environment.
    let token = <yourdiscordtokenhere>;

    // The Application Id is usually the Bot User Id.
    let application_id: u64 = <yourapplicationidhere>;

    // Build our client.
    let mut client = Client::builder(token)
        .event_handler(Handler)
        .register_songbird()
        .application_id(application_id)
        .await
        .expect("Error creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
