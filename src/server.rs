mod event;
use clap::Parser;
use futures_util::StreamExt;
use std::time::Duration;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio_tungstenite::connect_async;

#[derive(Parser, Debug)]
#[command(name = "ticws-server")]
#[command(author = "Matthieu Totet <matthieu.totet@gmail.com>")]
#[command(version = include_str!("../.VERSION"))]
#[command(about = "Websocket relay server for Tic80 bytebattle based on bonzomatic protocol", long_about = "Please check that Client and Server version are aligned")]
pub struct TicwsServer {
    /// Room Name
    pub room: String,

    /// Handle Name
    pub handle: String,

    /// .dat Filepath
    pub file: String,

    /// Server host format ws://localhost.com
    #[arg(default_value_t = String::from("ws://drone.alkama.com"))]
    pub host: String,

    #[arg(default_value_t = String::from("9000"))]
    /// Port number
    pub port: String,
    
}

#[tokio::main]
async fn main() {
    let args = TicwsServer::parse();

    let connect_addr = format!("{}:{}/{}/{}", args.host, args.port, args.room, args.handle);
    println!("Connecting to {connect_addr}");

    let url = url::Url::parse(&connect_addr).unwrap();

    // Server don't care about sending stuff as it just listen and dump in file

    let (ws_stream, _) = connect_async(url.to_string()).await.expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (_, read) = ws_stream.split();

    let a = read.for_each(|message| async {
        let mut file = File::create(&args.file)
            .await
            .expect("File should be available");
        let data = message.unwrap().into_text().unwrap();
        if data.is_empty() { // Sometime there is some emty package, ping ? So we need to discard them to avoid raising error
            return;
        }
        let deserialized = serde_json::from_str(&data);
        if deserialized.is_ok() {
            let deserialized: event::Event = deserialized.unwrap();
            file.write_all(&(deserialized.data.as_bytes())).await.expect("Write in file");
        } else {
            eprintln!("Warning : serde_json failed ");
        }

    });
    a.await;
    loop {
        tokio::time::sleep(Duration::from_secs_f64(0.3f64)).await
    }
}
