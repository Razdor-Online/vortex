use crate::utils::server_utils;

mod utils;
use anyhow::Result;
use futures::SinkExt;
use signaling::packets::PacketC2S;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use utils::client_sender::ClientSender;

#[tokio::test]
async fn start_server_and_test() -> Result<()> {
    prepare_test_server!();
    let (mut ws_stream, _)  = prepare_test_client!();

    // gracefully close connection
    ws_stream.send("graceful".into()).await?;

    Ok(())
}

#[tokio::test]
async fn start_server_and_connect() -> Result<()> {
    prepare_test_server!();
    let (ws_stream, _)  = prepare_test_client!();

    let sender = ClientSender::new(ws_stream);

    sender.send(PacketC2S::Connect {
        room_id: "1".to_string(),
        token: "token".to_string(),
    }).await?;

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    Ok(())
}