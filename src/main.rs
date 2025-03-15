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
async fn send_message(ws_stream: &mut WebSocketStream<tokio::net::TcpStream>) {
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
    if ws_stream.send(Message::Text(message.into())).await.is_err() {
        println!("Error sending message to client.");
    }
}

fn generate_random_number() -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..100)  // Random number between 0 and 100
}

async fn handle_client(mut ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>) {
    // let mut candle_stick_data = CandleStick {
    //     ts: 0,
    //     o: 0.0,
    //     h: 0.0,
    //     l: 0.0,
    //     c: 0.0,
    //     v: 0.0,
    // };

    // Create a timer to periodically send candle data
    let mut interval = time::interval(Duration::from_secs(1)); // Send data every 60 seconds

    loop {
        // Send message every interval
        interval.tick().await;

        send_message(&mut ws_stream).await;
        send_message(&mut ws_stream).await;
        send_message(&mut ws_stream).await;
        send_message(&mut ws_stream).await;

        // Prepare a sample candlestick message (Binance-style)
        // let kline = Kline {
        //     open_time: candle_stick_data.ts,
        //     close: candle_stick_data.c.to_string(),
        //     open: candle_stick_data.o.to_string(),
        //     high: candle_stick_data.h.to_string(),
        //     low: candle_stick_data.l.to_string(),
        //     volume: candle_stick_data.v.to_string(),
        // };

        // let ws_message = WebSocketMessage { kline };

        // // Send the message to the client
        // let message = serde_json::to_string(&ws_message).unwrap();

        // // Send the message as Text
        // if  ws_stream.send(Message::Text(message.into())).await.is_err() {
        //     println!("Error sending message to client.");
        //     break; // If sending fails, break the loop (or handle as necessary)
        // }

        

        // // Update the candle data (this could be done with real data in practice)
        // candle_stick_data.ts += 60; // Increment timestamp for simplicity
        // candle_stick_data.o = 35000.0; // Example open price
        // candle_stick_data.h = 35500.0; // Example high price
        // candle_stick_data.l = 34000.0; // Example low price
        // candle_stick_data.c = 34500.0; // Example close price
        // candle_stick_data.v = 1000.0;  // Example volume
    }

    // Gracefully close the connection after finishing
    let close_frame = CloseFrame {
        code: tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Normal,
        reason: "Close".into(), // Empty reason
    };
    ws_stream.close(Some(close_frame)).await.unwrap();
    println!("Connection closed.");
}