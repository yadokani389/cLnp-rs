use anyhow::Error;
use dotenvy::dotenv;
use poise::serenity_prelude::*;
use std::env;

type Context<'a> = poise::Context<'a, Data, Error>;

pub struct Data {}

mod commands;

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to read .env file");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::non_privileged();

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::tic_tac_toe::tic_tac_toe()],
            ..Default::default()
        })
        .setup(move |ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    client
        .expect("Failed to create client")
        .start()
        .await
        .expect("Failed to start client");
}
