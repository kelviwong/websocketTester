use tokio::net::TcpListener;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use futures::stream::{StreamExt};
use futures::SinkExt;
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio::time::Duration;
use tokio::time;
use tokio_tungstenite::{accept_async, tungstenite::protocol::{CloseFrame}};
use tokio_tungstenite::WebSocketStream;
use rand::Rng;



#[derive(Serialize)]
struct KlineData {
    e: String,  // Event type
    E: u64,     // Event time
    s: String,  // Symbol
    k: KlineInfo,
}

#[derive(Serialize)]
struct KlineInfo {
    t: u64,      // Start time
    T: u64,      // End time
    s: String,   // Symbol
    i: String,   // Interval
    f: u64,      // First trade id
    L: u64,      // Last trade id
    o: String,   // Open price
    c: String,   // Close price
    h: String,   // High price
    l: String,   // Low price
    v: String,   // Volume
    n: u64,      // Number of trades
    x: bool,     // Is closed
    q: String,   // Quote volume
    V: String,   // Volume in the market
    Q: String,   // Quote in the market
    B: String,   // Binance market
}

#[tokio::main]
async fn main() {
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();

    println!("WebSocket server running on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let ws_stream = accept_async(stream)
            .await
            .expect("Error during WebSocket handshake");

        // Handle each client connection in a separate task
        tokio::spawn(handle_client(ws_stream));
    }
}

// Function to send message to WebSocket client
async fn send_message(ws_stream: &mut WebSocketStream<tokio::net::TcpStream>) -> Result<(), tokio_tungstenite::tungstenite::Error> {
    let mut kline_data = KlineData {
        e: "kline".to_string(),
        E: 1742037122019,
        s: "BTCUSDT".to_string(),
        k: KlineInfo {
            t: 1742037120000,
            T: 1742037179999,
            s: "BTCUSDT".to_string(),
            i: "1m".to_string(),
            f: 4718215945,
            L: 4718215951,
            o: "83965.27000000".to_string(),
            c: "83965.27000000".to_string(),
            h: "83965.28000000".to_string(),
            l: "83965.27000000".to_string(),
            v: "0.04597000".to_string(),
            n: 7,
            x: false,
            q: "3859.88353480".to_string(),
            V: "0.00729000".to_string(),
            Q: "612.10689120".to_string(),
            B: "0".to_string(),
        },
    };

    let random_number = generate_random_number();

    kline_data.k.t += random_number;

    // Serialize the KlineData struct to JSON
    let message = serde_json::to_string(&kline_data).expect("Failed to serialize message");

    // Send the message as a WebSocket message
    match ws_stream.send(Message::Text(message.into())).await {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Failed to send message: {}", e);
            Err(e) // Return the error for further handling
        }
    }
}

fn generate_random_number() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..100)  // Random number between 0 and 100
}

async fn handle_client(mut ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>) {
    // Create a timer to periodically send candle data
    let mut interval = time::interval(Duration::from_secs(1)); // Send data every 60 seconds

    loop {
        tokio::select! {
            _ = interval.tick() => {
                // Send message every interval
                if let Err(e) = send_message(&mut ws_stream).await {
                    println!("Client closed the connection.");
                    break;
                }
            }

            // Check if a message is received
            Some(Ok(message)) = ws_stream.next() => {
                match message {
                    Message::Close(_) => {
                        // Gracefully handle client closing the connection
                        println!("Client closed the connection.");
                        break;  // Break the loop to stop sending messages
                    }
                    Message::Ping(_) | Message::Pong(_) => {
                        // Handle Ping and Pong messages if necessary
                    }
                    _ => {
                        // Handle other message types if needed
                    }
                }
            }
            //  // If the connection is dropped, break the loop
            //  None => {
            //     println!("Client connection lost.");
            //     break;  // Break the loop to stop sending messages
            // }
        }
    }

    // // Gracefully close the connection after finishing
    // let close_frame = CloseFrame {
    //     code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Normal,
    //     reason: "Close".into(), // Empty reason
    // };
    // ws_stream.close(Some(close_frame)).await.unwrap();
    // println!("Connection closed.");
}