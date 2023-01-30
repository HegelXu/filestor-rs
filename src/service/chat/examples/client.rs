// use anyhow::Result;
// use chat::client::Client;
// use std::env;
// use tokio::io::{self, AsyncBufReadExt};

// #[tokio::main]
// async fn main() -> Result<()> {
//     tracing_subscriber::fmt::init();
//     let username = env::var("USERNAME").unwrap();
//     let mut client = Client::new(username).await;
//     client.login().await?;
//     client.get_message().await?;

//     let mut stdin = io::BufReader::new(io::stdin()).lines();

//     while let Ok(Some(line)) = stdin.next_line().await {
//         // let mut line = String::new();
//         // stdin.read_line(&mut line).await?;
//         // let line = line.trim();
//         // if line == "quit" {
//         //     break;
//         // }
//         client.send_message("lobby", line).await?;
//     }

//     Ok(())
// }


fn main() {
    println!("a");
}