use std::env;
use std::time::Duration;

use anyhow::Result;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::{response::IntoResponse, routing::get, Router};

use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt};
use rust_web_sockets::WsMessage;
use tokio::net::TcpListener;
use tokio::select;
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:8080".to_string());

    let listener = TcpListener::bind(&addr)
        .await
        .expect("Error acquiring listener from socket");

    let app = Router::new()
        .route("/ws", get(ws_handler))
        .nest_service("/", ServeDir::new("public"))
        .into_make_service();

    axum::serve(listener, app).await?;

    Ok(())
}

async fn ws_handler(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.on_upgrade(handle_socket_split)
}

async fn handle_socket_split(socket: WebSocket) {
    let (writer, reader) = socket.split();

    let (tx, rx) = channel::<WsMessage>(32);

    tokio::spawn(async { ws_reader(reader, tx).await });
    tokio::spawn(async { ws_writer(writer, rx).await });
}

/// The web socket reader that reads messages from the client,
/// manages state and forwards messages to the writer task who will
/// handle the different message cases sent.
async fn ws_reader(mut receiver: SplitStream<WebSocket>, channel: Sender<WsMessage>) {
    while let Some(Ok(message)) = receiver.next().await {
        let wsm: WsMessage = message.into();
        channel.send(wsm).await.expect("Error sending in channel");
    }
}

/// The web socket writer handler that deals with messages
/// sent from the web client.
async fn ws_writer(
    mut writer: SplitSink<WebSocket, Message>,
    mut channel: Receiver<WsMessage>,
) -> Result<()> {
    let mut interval = tokio::time::interval(Duration::from_secs(3));

    loop {
        select! {
            msg = channel.recv() => {
                writer.send(Message::Text("Echoing message: ".to_string() + &msg.unwrap().to_string())).await?;
            }
            _ = interval.tick() => {
                println!("Tick!");
            }
        }
    }
}
