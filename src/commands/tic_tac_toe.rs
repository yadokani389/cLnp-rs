use anyhow::Error;
use itertools::Itertools;
use poise::serenity_prelude::*;
use poise::CreateReply;
use std::time::Duration;

use crate::Context;

#[poise::command(slash_command)]
pub async fn tic_tac_toe(ctx: Context<'_>) -> Result<(), Error> {
    let mut board = ["\0"; 9];

    let make_components = |board: &[&str; 9], end: bool| {
        let mut components = Vec::new();
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
        components
    };

    let mut m = ctx
        .send(
            CreateReply::default()
                .content("O's turn!")
                .components(make_components(&board, false)),
        )
        .await?
        .into_message()
        .await?;

    let mut count = 0;
    loop {
        let interaction = match m
            .await_component_interaction(ctx)
            .timeout(Duration::from_secs(10))
            .await
        {
            Some(x) => x,
            None => {
                // Timeout
                m.edit(
                    &ctx,
                    EditMessage::default()
                        .content("Timed out")
                        .components(make_components(&board, true)),
                )
                .await?;
                return Ok(());
            }
        };

        let pos = interaction.data.custom_id.parse::<usize>()?;
        board[pos] = if count % 2 == 0 { "O" } else { "X" };

        let mut response = CreateInteractionResponseMessage::default().content(if count % 2 == 0 {
            "X's turn"
        } else {
            "O's turn"
        });

        let mut end = false;
        {
            // Check for winner
            let mut winner = "\0";
            for i in 0..3 {
                if board
                    .iter()
                    .skip(i * 3)
                    .take(3)
                    .tuple_windows()
                    .all(|(prev, next)| prev == next)
                {
                    winner = board[i * 3];
                }
                if board
                    .iter()
                    .skip(i)
                    .step_by(3)
                    .tuple_windows()
                    .all(|(prev, next)| prev == next)
                {
                    winner = board[i];
                }
            }
            if board
                .iter()
                .step_by(4)
                .tuple_windows()
                .all(|(prev, next)| prev == next)
            {
                winner = board[0];
            }
            if board
                .iter()
                .skip(2)
                .step_by(2)
                .take(3)
                .tuple_windows()
                .all(|(prev, next)| prev == next)
            {
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

        interaction
            .create_response(
                &ctx,
                CreateInteractionResponse::UpdateMessage(
                    response.components(make_components(&board, end)),
                ),
            )
            .await?;

        if end {
            return Ok(());
        }
        count += 1;
    }
}
