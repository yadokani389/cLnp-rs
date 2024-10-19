pub struct Handler;

use serenity::all::{CreateActionRow, EditMessage, EventHandler};
use serenity::async_trait;
use serenity::builder::{
    CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage,
};
use serenity::model::prelude::*;
use serenity::prelude::*;
use std::time::Duration;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!tic_tac_toe" {
            let mut board = ["\0"; 9];

            let mut components = Vec::new();
            for i in 0..3 {
                let mut row = Vec::new();
                for j in 0..3 {
                    row.push(CreateButton::new(format!("{}", i * 3 + j)).label(board[i * 3 + j]));
                }
                components.push(CreateActionRow::Buttons(row));
            }
            let mut m = msg
                .channel_id
                .send_message(
                    &ctx,
                    CreateMessage::new()
                        .content("O's turn!")
                        .components(components),
                )
                .await
                .unwrap();

            let mut count = 0;
            loop {
                let interaction = match m
                    .await_component_interaction(&ctx.shard)
                    .timeout(Duration::from_secs(10))
                    .await
                {
                    Some(x) => x,
                    None => {
                        // Timeout
                        components = Vec::new();
                        for i in 0..3 {
                            let mut row = Vec::new();
                            for j in 0..3 {
                                row.push(
                                    CreateButton::new(format!("{}", i * 3 + j))
                                        .label(board[i * 3 + j])
                                        .disabled(true),
                                );
                            }
                            components.push(CreateActionRow::Buttons(row));
                        }
                        m.edit(
                            &ctx,
                            EditMessage::default()
                                .content("Timed out")
                                .components(components),
                        )
                        .await
                        .unwrap();
                        return;
                    }
                };

                let pos = interaction.data.custom_id.parse::<usize>().unwrap();
                board[pos] = if count % 2 == 0 { "O" } else { "X" };

                let mut response =
                    CreateInteractionResponseMessage::default().content(if count % 2 == 0 {
                        "X's turn"
                    } else {
                        "O's turn"
                    });

                let mut end = false;
                {
                    // Check for winner
                    let mut winner = "\0";
                    for i in 0..3 {
                        if board[i * 3] == board[i * 3 + 1]
                            && board[i * 3 + 1] == board[i * 3 + 2]
                            && board[i * 3] != "\0"
                        {
                            winner = board[i * 3];
                        }
                        if board[i] == board[3 + i]
                            && board[3 + i] == board[6 + i]
                            && board[i] != "\0"
                        {
                            winner = board[i];
                        }
                    }
                    if board[0] == board[4] && board[4] == board[8] && board[0] != "\0" {
                        winner = board[0];
                    }
                    if board[2] == board[4] && board[4] == board[6] && board[6] != "\0" {
                        winner = board[2];
                    }
                    if winner != "\0" {
                        response = response.content(format!("{} wins!", winner));
                        end = true;
                    }
                    if count == 8 {
                        response = response.content("draw!");
                        end = true;
                    }
                }

                components = Vec::new();
                for i in 0..3 {
                    let mut row = Vec::new();
                    for j in 0..3 {
                        row.push(
                            CreateButton::new(format!("{}", i * 3 + j))
                                .label(board[i * 3 + j])
                                .disabled(board[i * 3 + j] != "\0" || end),
                        );
                    }
                    components.push(CreateActionRow::Buttons(row));
                }

                interaction
                    .create_response(
                        &ctx,
                        CreateInteractionResponse::UpdateMessage(response.components(components)),
                    )
                    .await
                    .unwrap();

                if end {
                    return;
                }
                count += 1;
            }
        }
    }
}
