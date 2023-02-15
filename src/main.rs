mod commands;

use std::env;
use dotenv::dotenv;

use serenity::async_trait;
use serenity::model::prelude::{Ready, GuildId};
use serenity::model::prelude::application::interaction::{Interaction, InteractionResponseType};
use serenity::prelude::*;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx : Context, interaction: Interaction){
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "ping" => commands::ping::run(&command.data.options),
                _ => "not implemented".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response|{
                    response.kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                }).await {
                    println!("cannot respond to command: {}", why);
                }
        }
    }

    async fn ready(&self, ctx : Context, ready: Ready){
        println!("{} is now connected.", ready.user.name);

        let guild_id = GuildId(
            env::var("GUILD_ID").expect("Expected guild id in environment").parse().expect("Build id must be an integer"),
        );  

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands|{
            commands
                .create_application_command(|command| commands::ping::register(command))
        }).await;

        println!("Added commands: {:#?}", commands);

        // let guild_command = Command::create_global_application_command(&ctx.http, |command| {
        //     commands::global_test::register(command);
        // }).await;
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    

    // Login with a bot token from the environment
    let token = env::var("DISCORD_TOKEN").unwrap();
    println!("{}", token);
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, GatewayIntents::empty())
    .event_handler(Handler)
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
