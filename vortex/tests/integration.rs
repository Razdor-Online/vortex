use crate::utils::server_utils;

mod utils;
use anyhow::Result;
use futures::{SinkExt};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;

#[tokio::test]
async fn start_server_and_test() -> Result<()> {
    server_utils::init().await;
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let req = "ws://127.0.0.1:8080".into_client_request()?;

    let (mut ws_stream, _) = tokio_tungstenite::connect_async(req).await?;
    // gracefully close connection
    ws_stream.send("graceful".into()).await?;

    Ok(())
}