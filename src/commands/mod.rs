use serenity::async_trait;
use serenity::model::application::Interaction;
use serenity::model::gateway::Ready;
use serenity::model::id::GuildId;
use serenity::prelude::*;
use std::env;

pub mod tic_tac_toe;

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            println!("Received command interaction: {command:#?}");

            match command.data.name.as_str() {
                "tic_tac_toe" => tic_tac_toe::run(ctx, command).await,
                _ => {}
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        let guild_id = GuildId::new(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );

        let commands = guild_id
            .set_commands(&ctx.http, vec![tic_tac_toe::register()])
            .await;

        println!("I now have the following guild slash commands: {commands:#?}");
    }
}
