use std::net::SocketAddr;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::sync::mpsc::{self, Sender};
const SERVER: &str = "127.0.0.1:8080";

type ClientWriter = OwnedWriteHalf;
type MatchRequest = (ClientWriter, Sender<(ClientWriter, u8, u8)>);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind(SERVER).await?;
    println!("listening on :8080");

    let (matcher_tx, mut matcher_rx) = mpsc::channel::<MatchRequest>(64);

    tokio::spawn(async move {
        let mut pending: Vec<MatchRequest> = Vec::new();
        while let Some((writer, sender)) = matcher_rx.recv().await {
            pending.push((writer, sender));
            let l = pending.len();
            println!("Matcher:  len: {l}");
            if pending.len() >= 2 {
                // Take two clients
                let (writer_a, sender_a) = pending.pop().unwrap();
                let (writer_b, sender_b) = pending.pop().unwrap();

                // Exchange writers
                let _ = sender_a.send((writer_b, 1, 1)).await; // A gets B's writer, (true, circle)
                let _ = sender_b.send((writer_a, 0, 0)).await; // B gets A's writer
            }
        }
    });

    loop {
        let (socket, addr) = listener.accept().await?;
        let matcher_tx = matcher_tx.clone();
        println!("{addr} Connected!");
        tokio::spawn(async move {
            let (mut reader, writer) = socket.into_split();
            let (tx_to_me, mut rx_from_matcher) = mpsc::channel::<(ClientWriter, u8, u8)>(1);

            let mut buf = [b'z'];
            reader.read(&mut buf).await.expect("Error reading");
            println!("Client {addr}: step 1: {buf:?}");
            if buf[0] == 1 as u8 {
                // multiplayer logic
                // send message to matcher
                // get opponentes writer and give self writer
                matcher_tx
                    .send((writer, tx_to_me.clone()))
                    .await
                    .expect("Error in sending");
                println!("Client: match requested!");

                let received = rx_from_matcher.recv().await.expect("receive failed");
                println!("Client: Match found!");
                let mut writer_opponent = received.0;
                let your_turn = received.1;
                let player_type = received.2;
                writer_opponent
                    .write(&[your_turn, player_type])
                    .await
                    .expect("Write faield");
                loop {
                    let buf = &mut [0; 3];
                    reader.read_exact(buf).await.expect("read failed");
                    println!("Client: Buffer after read: {buf:?}");
                    if let Err(e) = writer_opponent.write(buf).await {
                        eprintln!("Write error (client probably disconnected): {}", e);
                        break; // exit the loop and let the task finish
                    };
                }
            } else if buf[0] == 2 {
                //Single player
            }
        });
    }
}

// call client thread -> calls matcher -> matcher matchs and makes a gaming thread
