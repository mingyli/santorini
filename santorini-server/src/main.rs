use async_std::net;
use async_tungstenite::tungstenite;
use futures::{SinkExt, StreamExt};

use santorini_common::objects;

async fn handle_connection(stream: net::TcpStream) -> async_tungstenite::tungstenite::Result<()> {
    let ws_stream = async_tungstenite::accept_async(stream).await.unwrap();
    let (mut write, mut _read) = ws_stream.split();

    let state = objects::state::State::default();
    for _ in 0..4 {
        write
            .send(tungstenite::Message::Binary(
                bincode::serialize(&state).unwrap(),
            ))
            .await?;
    }
    Ok::<(), _>(())
}

async fn run() {
    const ADDRESS: &str = "localhost:8000";
    let listener = net::TcpListener::bind(&ADDRESS).await.unwrap();
    println!("{}", ADDRESS);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().unwrap();
        println!("{}", peer);
        async_std::task::spawn(handle_connection(stream));
    }
}

fn main() {
    async_std::task::block_on(run());
}
