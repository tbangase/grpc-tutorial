
use tokio::time;
use tonic::Request;
use std::time::Duration;
use grpc_server::route_guide_rpc::route_guide;
use route_guide::{
    route_guide_client::RouteGuideClient,
    Point, RouteNote,
};

use futures_util::StreamExt;

use rand::Rng;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    std::env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();
    // Serverとのコネクションを作成する。
    let mut client = RouteGuideClient::connect(
        "http://[::1]:10000"
    ).await?;

    let start = time::Instant::now();

    // サーバーに渡すStreamの作成
    let (tx, rx) = std::sync::mpsc::channel();

    tracing::debug!("Spawning sender and receiver");

    let outbound = async_stream::stream! {
        // インターバルの設定
        let mut latency = time::interval(Duration::from_secs(1));

        println!("outbound function called.");

        for val in rx {
            // client_streamで受信する
            // インターバルを待つ
            println!("Waiting for heavy function call");
            let time = latency.tick().await;

            // 新しいRouteNoteの作成
            let elapsed = time.duration_since(start);
            let note = RouteNote {
                location: Some(Point{
                    latitude: 409146138 + elapsed.as_millis() as i32,
                    longitude: -746188906,
                }),
                message: format!("at {:?}: {val}", elapsed),
            };

            yield note;
        }

    };

    // 1. 別スレッドでメッセージを送信する
    std::thread::spawn(move || {
        let messages = vec![
            format!("First message"),
            format!("Second message"),
            format!("Third message"),
            format!("Fourth message"),
            format!("Fifth message"),
            format!("Sixth message"),
            format!("Seventh message"),
            format!("Eighth message"),
            format!("Ninth message"),
            format!("Tenth message"),
        ];

        let mut rng = rand::thread_rng();
        let mut message_iter = messages.iter();

        while let Some(message) = message_iter.next() {
            std::thread::sleep(Duration::from_millis(rng.gen_range(500..2000)));
            println!("Sending message: {message}");
            tx.send(message.clone()).unwrap();
        }
    });

    let response = client.route_chat(Request::new(outbound)).await?;

    let mut inbound = response.into_inner();
    
    tracing::debug!("Waiting for inbound message");

    while let Some(note) = inbound.next().await {
        println!("NOTE = {note:?}");
    }

    Ok(())
}