use std::net::SocketAddr;

use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader};
use tokio::net::tcp::OwnedWriteHalf;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio::sync::mpsc::{self, Sender};
const SERVER: &str = "127.0.0.1:8080";

async fn handle_client(
    socket: TcpStream,
    addr: SocketAddr,
    tx: broadcast::Sender<String>,
    mut rx: broadcast::Receiver<String>,
) {
    let (reader, mut writer) = socket.into_split();
    let mut lines = BufReader::new(reader).lines();

    println!("{addr} connected");

    loop {
        tokio::select! {
            result = lines.next_line() => {
                match result {
                    Ok(Some(line)) => { tx.send(format!("{addr}: {line}")).ok(); }
                    _ => break,
                }
            }
            result = rx.recv() => {
                match result {
                    Ok(msg) => { writer.write_all(format!("{msg}\n").as_bytes()).await.ok(); }
                    Err(_) => break,
                }
            }
        }
    }

    println!("{addr} disconnected");
}

type ClientWriter = OwnedWriteHalf;
type MatchRequest = (ClientWriter, Sender<ClientWriter>);

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind(SERVER).await?;
    println!("listening on :8080");

    let (matcher_tx, mut matcher_rx) = mpsc::channel::<MatchRequest>(64);

    tokio::spawn(async move {
        let mut pending: Vec<MatchRequest> = Vec::new();
        while let Some((writer, sender)) = matcher_rx.recv().await {
            pending.push((writer, sender));
            if pending.len() >= 2 {
                // Take two clients
                let (writer_a, sender_a) = pending.pop().unwrap();
                let (writer_b, sender_b) = pending.pop().unwrap();

                // Exchange writers
                let _ = sender_a.send(writer_b).await; // A gets B's writer
                let _ = sender_b.send(writer_a).await; // B gets A's writer
            }
        }
    });

    loop {
        let (socket, addr) = listener.accept().await?;
        let matcher_tx = matcher_tx.clone();

        tokio::spawn(async move {
            let (mut reader, mut writer) = socket.into_split();
            let (tx_to_me, mut rx_from_matcher) = mpsc::channel::<ClientWriter>(1);

            loop {
                let mut buf = [b'z'];
                reader.read(&mut buf).await.expect("Error reading");

                if buf[0] == 1 as u8 {
                    // multiplayer logic
                    // send message to matcher
                    // get opponentes writer and give self writer
                    matcher_tx
                        .send((writer, tx_to_me.clone()))
                        .await
                        .expect("Error in sending");
                } else if buf[0] == 2 {
                    //Single player
                }
            }
        });
    }
}

// call client thread -> calls matcher -> matcher matchs and makes a gaming thread
