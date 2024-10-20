use serenity::all::{CommandInteraction, Context, CreateActionRow, EditMessage};
use serenity::builder::{
    CreateButton, CreateCommand, CreateInteractionResponse, CreateInteractionResponseMessage,
};
use std::time::Duration;

pub async fn run(ctx: Context, command: CommandInteraction) {
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

    command
        .create_response(
            &ctx,
            CreateInteractionResponse::Message(
                CreateInteractionResponseMessage::new()
                    .content("O's turn!")
                    .components(make_components(&board, false)),
            ),
        )
        .await
        .unwrap();

    let mut m = command.get_response(&ctx).await.unwrap();

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
                m.edit(
                    &ctx,
                    EditMessage::default()
                        .content("Timed out")
                        .components(make_components(&board, true)),
                )
                .await
                .unwrap();
                return;
            }
        };

        let pos = interaction.data.custom_id.parse::<usize>().unwrap();
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
                if board[i * 3] == board[i * 3 + 1]
                    && board[i * 3 + 1] == board[i * 3 + 2]
                    && board[i * 3] != "\0"
                {
                    winner = board[i * 3];
                }
                if board[i] == board[3 + i] && board[3 + i] == board[6 + i] && board[i] != "\0" {
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

        interaction
            .create_response(
                &ctx,
                CreateInteractionResponse::UpdateMessage(
                    response.components(make_components(&board, end)),
                ),
            )
            .await
            .unwrap();

        if end {
            return;
        }
        count += 1;
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("tic_tac_toe").description("Start game")
}