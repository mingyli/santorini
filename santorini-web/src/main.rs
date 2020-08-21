use async_tungstenite::tungstenite;
use futures::StreamExt;

use santorini_common::objects;

async fn run() -> async_tungstenite::tungstenite::Result<()> {
    const ADDRESS: &str = "ws://localhost:8000/socket";
    let (ws_stream, response) = async_tungstenite::async_std::connect_async(ADDRESS)
        .await
        .unwrap();
    for (ref header, value) in response.headers() {
        println!("{} {:?}", header, value);
    }

    let (mut _write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        let msg = msg?;
        match msg {
            tungstenite::Message::Binary(bytes) => {
                let state: objects::state::State = bincode::deserialize(&bytes).unwrap();
                println!("{:?}", state);
            }
            _ => println!("{}", msg),
        }
    }
    Ok(())
}

fn main() {
    async_std::task::block_on(run()).unwrap();
}
