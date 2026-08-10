use ::mpsc::Receiver;
use macroquad::prelude::*;
use rig::providers::gemini::interactions_api::UrlContextCallArguments;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;
use tictactoe::Board;
use tictactoe::CellState;
use tictactoe_ai_chatroom_rust_workshop::tictactoe::{self};
const WINDOW_W: f32 = 800.0;
const WINDOW_H: f32 = 600.0;
const BOARD_SIZE: f32 = 500.0;
const BOARD_X: f32 = WINDOW_W / 2.0 - BOARD_SIZE / 2.0;
const BOARD_Y: f32 = 50.0;
const SERVER: &str = "127.0.0.1:8080";
enum GameState {
    MultiPlayer,
    SinglePlayer,
    Menu(u8),
    Waiting,
    ShowResult(bool),
}

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
    let mut stream;
    let mut writer;
    let (tx_mp, rx_mp) = mpsc::channel::<(bool, CellState)>(); //(turn, player_type) for multiplayer mode
    let mut line_reader;
    let mut reader;
    let mut game_state = GameState::Menu(0);
    let menu_options = ["Exit", "MultiPlayer", "Single Player vs LLM"];
    let mut board = Board::new(BOARD_X, BOARD_Y, BOARD_SIZE);
    let waiting_text = "Finding Opponent...";
    let mut win_text: &str;
    let mut your_turn: bool = false; // TODO fix
    let mut player_type: CellState; // TODO
    //let mut handle;
    loop {
        let dt = get_frame_time();
        // TODO, update board server
        match &mut game_state {
            GameState::ShowResult(won) => {
                if *won {
                    win_text = "You Win! :), Press Enter...";
                } else {
                    win_text = "You Lose! :(, Press Enter...";
                }
                clear_background(BLACK);
                if is_key_pressed(KeyCode::Enter) {
                    // TODO
                }
                let dims = measure_text(win_text, None, 36, 1.0);
                let text_x = WINDOW_W / 2.0 - dims.width / 2.0;
                let text_y = WINDOW_H / 2.0;
                draw_text(win_text, text_x, text_y, 24.0, WHITE);
            }
            GameState::Waiting => {
                clear_background(BLACK);
                // TODO, get data from server that opponent found
                // update gamestate
                //
                //
                match rx_mp.try_recv() {
                    Ok(msg) => {
                        your_turn = msg.0;
                        player_type = msg.1;
                        game_state = GameState::MultiPlayer;
                    }
                    Err(_) => {} // nothing waiting
                }
                let dims = measure_text(waiting_text, None, 36, 1.0);
                let text_x = WINDOW_W / 2.0 - dims.width / 2.0;
                let text_y = WINDOW_H / 2.0;
                draw_text(waiting_text, text_x, text_y, 24.0, WHITE);
            }
            GameState::Menu(sel) => {
                clear_background(BLACK);
                let mut select = *sel;
                if is_key_pressed(KeyCode::Down) {
                    *sel = *&sel.saturating_sub(1);
                    select = *sel;
                } else if is_key_pressed(KeyCode::Up) {
                    *sel = if *sel == 2 { 2 } else { *sel + 1 };
                    select = *sel;
                } else if is_key_pressed(KeyCode::Enter) {
                    if *sel == 0 {
                        // TODO, NETWORK SHUTDOWN SIGNAL
                        std::process::exit(0);
                    } else if *sel == 2 {
                        // TODO, SINGLE PLAYER
                        game_state = GameState::SinglePlayer;
                    } else {
                        // TODO, MULTIPLAER
                        // Send ginal to server
                        drop(stream);
                        stream = TcpStream::connect(SERVER).expect("could not connect to server");
                        writer = stream.try_clone().expect("clone failed");
                        line_reader = BufReader::new(stream.try_clone().expect("clone failed"));
                        reader = stream.try_clone().expect("clone failed");
                        // reader.set_nonblocking(true);
                        let tx = tx_mp.clone();
                        let mut stream_cp = stream.try_clone().expect("clone failed in menu");

                        thread::spawn(move || {
                            stream_cp.write_all(&[1]).expect("Send failed");
                            let buf = &mut [0, 0];
                            stream_cp.read(buf).expect("Read failed");
                            let your_turn = if buf[0] == 1 { true } else { false };
                            let cs = if buf[1] == 1 {
                                CellState::Circle
                            } else {
                                CellState::Cross
                            };
                            tx.send((your_turn, cs)).expect("Sent failed");
                        });

                        game_state = GameState::Waiting;
                    }
                }

                let mut y_offset = 0.0_f32;
                // *sel = 1;
                for i in 0..menu_options.len() {
                    let dims = measure_text(menu_options[i], None, 24, 1.0);
                    let text_x = WINDOW_W / 2.0 - dims.width / 2.0;
                    let text_y = WINDOW_H / 2.0 - y_offset;

                    if i == select as usize {
                        let pad = 10.0;
                        draw_rectangle(
                            text_x - pad,
                            text_y - pad - 14.0,
                            dims.width + pad * 2.0,
                            dims.height + pad * 2.0,
                            DARKGRAY,
                        );
                    }

                    let color = if i == select as usize { BLACK } else { WHITE };
                    draw_text(menu_options[i], text_x, text_y, 24.0, color);

                    y_offset += dims.height + 25.0_f32;
                }
            }
            GameState::MultiPlayer => {
                if your_turn {
                    // TODO
                } else {
                    // TODO
                }

                board.draw();
            }
            GameState::SinglePlayer => {
                if your_turn {
                    // TODO
                } else {
                    // TODO
                }
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mx, my) = mouse_position();
                    match board.cell_check(mx, my) {
                        None => {}
                        Some(cell) => {
                            // if let CellState::Empty
                            // board.update_cell(cell.0, cell.1);
                        }
                    }
                }
                board.draw();
            }
        }

        next_frame().await;
    }
}
