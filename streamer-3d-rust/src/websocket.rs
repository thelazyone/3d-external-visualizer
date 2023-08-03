use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::protocol::CloseFrame;
use std::borrow::Cow;
use std::sync::mpsc;
use futures::{SinkExt, StreamExt};


// Websocket functions:

pub async fn send_mesh_ws(
    mut websocket: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    model_data: String,
    tx: mpsc::Sender<()>
) -> Result<(), Box<dyn std::error::Error>> {

    websocket.send(Message::Text(model_data)).await?; // Use send method here

    let mut ack = None;
    while ack.is_none() {
        if let Some(Ok(msg)) = websocket.next().await { // Use the next method here
            if msg.is_text() && msg.to_text().unwrap() == "ACK" {
                ack = Some(());
            }
        }
    }

    let close_frame = CloseFrame {
        code: 1000.into(), // Normal closure code
        reason: Cow::Borrowed(""),
    };
    websocket.close(Some(close_frame)).await?; // Use close method with a CloseFrame

    tx.send(()).unwrap(); 
    Ok(())
}


pub async fn start_server_ws(model_data: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut stop_server = false;

    // Create a TCP listener
    let listener = TcpListener::bind("localhost:8765").await?;
    while !stop_server {
        // Accept an incoming TCP connection
        if let Ok((stream, _)) = listener.accept().await {
            // Upgrade the TCP connection to a WebSocket connection
            let websocket = accept_async(stream).await?;

            // Create a channel for sending messages
            let (tx, rx) = mpsc::channel();

            // Send the model data to the client
            send_mesh_ws(websocket, model_data.clone(), tx).await?;

            // Receive the signal (whether "ACK" received or not)
            if rx.recv().is_ok() {
                stop_server = true;
            }
        }
    }

    Ok(())
}