use macroquad::prelude::*;
use tictactoe::Board;
use tictactoe_ai_chatroom_rust_workshop::tictactoe;

const WINDOW_W: f32 = 800.0;
const WINDOW_H: f32 = 600.0;
const BOARD_SIZE: f32 = 500.0;
const BOARD_X: f32 = WINDOW_W / 2.0 - BOARD_SIZE / 2.0;
const BOARD_Y: f32 = 0.0;
fn window_conf() -> Conf {
    Conf {
        window_title: "TicTacToe".to_owned(),
        window_width: WINDOW_W as i32,
        window_height: WINDOW_H as i32,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    /* Run the game loop, stepping the simulation once per frame. */
    let board = Board::new(BOARD_X, BOARD_Y, BOARD_SIZE);
    loop {
        let dt = get_frame_time();

        next_frame().await;
    }
}
