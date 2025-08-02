mod event;
use clap::Parser;
use futures_util::{future, pin_mut, StreamExt};
use std::time::Duration;
use tokio::fs::{self, File};
use tokio::io::AsyncReadExt;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Parser, Debug)]
#[command(name = "ticws-client")]
#[command(author = "Matthieu Totet <matthieu.totet@gmail.com>")]
#[command(version = include_str!("../.VERSION"))]
#[command(
    about = "Websocket client relay for Tic80 bytebattle based on bonzomatic protocol",
    long_about = "Please check that Client and Server version are aligned"
)]
pub struct TicwsClient {
    /// Room Name
    pub room: String,

    /// Handle Name
    pub handle: String,

    /// .dat Filepath
    #[arg(default_value_t = String::from("showdown.dat"))]
    pub file: String,

    /// Server host format ws://localhost.com
    #[arg(default_value_t = String::from("ws://drone.alkama.com"))]
    pub host: String,

    #[arg(default_value_t = String::from("9000"))]
    /// Port number
    pub port: String,

    #[arg(short, long, default_value_t = 0.3f64)]
    /// Refresh Time in second
    pub refresh_time: f64,

    #[arg(short, long, default_value_t = false)]
    /// Only send new file when modified date change
    pub modified_time_update: bool,
}

#[tokio::main]
async fn main() {
    let args = TicwsClient::parse();

    let connect_addr = format!("{}:{}/{}/{}", args.host, args.port, args.room, args.handle);
    println!("Connecting to {connect_addr}");

    let url = url::Url::parse(&connect_addr).unwrap();

    let (stdin_tx, stdin_rx) = futures_channel::mpsc::unbounded();
    tokio::spawn(read_file(
        stdin_tx,
        args.file,
        args.refresh_time,
        format!("{}/{}", args.room, args.handle),
    ));

    let (ws_stream, _) = connect_async(url.to_string())
        .await
        .expect("Failed to connect");
    println!("WebSocket handshake has been successfully completed");

    let (write, read) = ws_stream.split();

    let stdin_to_ws = stdin_rx.map(Ok).forward(write);
    let ws_to_stdout = {
        read.for_each(|_| async {
            // We consume the message but don't do anything
            tokio::time::sleep(Duration::from_secs_f64(0.1f64)).await
        })
    };

    pin_mut!(stdin_to_ws, ws_to_stdout);
    future::select(stdin_to_ws, ws_to_stdout).await;
}

async fn read_file(
    tx: futures_channel::mpsc::UnboundedSender<Message>,
    in_file: String,
    refresh_time: f64,
    id: String,
) {
    let mut modified_timestamp = None;
    loop {
        let mut file = File::open(&in_file)
            .await
            .expect("File should be available");
        let metadata = fs::metadata(&in_file).await.unwrap();
        
        let current_modified_ts = metadata.modified().unwrap();
        if modified_timestamp.map(|x| current_modified_ts == x ).unwrap_or(false) {
            tokio::time::sleep(Duration::from_secs_f64(refresh_time)).await;
            continue;
        }

        modified_timestamp = Some(current_modified_ts);

        let mut content = String::new();
        let _ = file
            .read_to_string(&mut content)
            .await
            .expect("Should read the file from start");
        let msg = event::Event {
            s: "tic80".to_owned(),
            id: id.clone(),
            data: content,
        };
        let serialized_msg = serde_json::to_string(&msg).unwrap();
        tx.unbounded_send(Message::text(serialized_msg)).unwrap();
        tokio::time::sleep(Duration::from_secs_f64(refresh_time)).await
    }
}
