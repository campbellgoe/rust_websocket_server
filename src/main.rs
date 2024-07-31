use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::protocol::Message;
use anyhow::Result;
use futures_util::{SinkExt, StreamExt};
use tokio::sync::{mpsc, RwLock};
use std::collections::HashMap;
use std::sync::Arc;

type Tx = mpsc::UnboundedSender<Message>;
type Channels = Arc<RwLock<HashMap<String, Vec<Tx>>>>;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "127.0.0.1:8080".to_string();
    let listener = TcpListener::bind(&addr).await?;
    let channels: Channels = Arc::new(RwLock::new(HashMap::new()));
    println!("WebSocket server started on ws://{}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let channels = channels.clone();
        tokio::spawn(handle_connection(stream, channels));
    }

    Ok(())
}

async fn handle_connection(stream: tokio::net::TcpStream, channels: Channels) -> Result<()> {
    let ws_stream = accept_async(stream).await?;
    println!("Client connected");
    let (tx, rx) = mpsc::unbounded_channel();
    let (mut ws_sink, mut ws_stream) = ws_stream.split();

    tokio::spawn(async move {
        while let Some(msg) = ws_stream.next().await {
            if let Ok(msg) = msg {
                if msg.is_text() {
                    let received_text = msg.to_text().unwrap();
                    println!("Received message: {}", received_text);
                    handle_message(received_text, tx.clone(), channels.clone()).await;
                }
            }
        }
        println!("Client disconnected");
    });

    tokio::spawn(async move {
        let mut rx = rx;
        while let Some(msg) = rx.recv().await {
            ws_sink.send(msg).await.unwrap();
        }
    });

    Ok(())
}

async fn handle_message(message: &str, tx: Tx, channels: Channels) {
    let parts: Vec<&str> = message.splitn(3, ' ').collect();
    if parts.len() < 2 {
        return;
    }

    let command = parts[0];
    let channel = parts[1].to_string();

    match command {
        "JOIN" => {
            let mut chs = channels.write().await;
            let clients = chs.entry(channel.clone()).or_insert_with(Vec::new);
            clients.push(tx);
            println!("Client joined room: {}", channel);
        }
        "LEAVE" => {
            let mut chs = channels.write().await;
            if let Some(clients) = chs.get_mut(&channel) {
                clients.retain(|client| !client.same_channel(&tx));
                println!("Client left room: {}", channel);
            }
        }
        "MSG" if parts.len() == 3 => {
            let msg = parts[2];
            println!("Message in room {}: {}", channel, msg);
            let chs = channels.read().await;
            if let Some(clients) = chs.get(&channel) {
                for client in clients {
                    let _ = client.send(Message::Text(msg.to_string()));
                }
            }
        }
        _ => {
            println!("Unknown command: {}", message);
        }
    }
}