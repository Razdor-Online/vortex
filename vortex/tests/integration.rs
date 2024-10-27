use crate::utils::server_utils;

mod utils;
use anyhow::Result;
use api::negotiation;
use futures::StreamExt;
use signaling::packets::{PacketC2S, PacketS2C};
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use utils::client_sender::ClientSender;

#[tokio::test]
async fn start_server_and_test() -> Result<()> {
    prepare_test_server!();
    let (stream, _)   = prepare_test_client!();

    let(write, _read) = stream.split();
    let _write = ClientSender::new(write);

    Ok(())
}

#[tokio::test]
async fn start_server_and_connect() -> Result<()> {
    prepare_test_server!();
    let (stream, _)  = prepare_test_client!();
    let(write, mut read) = stream.split();
    let write = ClientSender::new(write);

    write.send(PacketC2S::Connect {
        room_id: "1".to_string(),
        token: "token".to_string(),
    }).await?;

    while let Some(msg) = read.next().await {
        let accept = PacketS2C::try_from(msg?).unwrap();
        assert_eq!(accept.to_json(),"{\"type\":\"Accept\",\"available_tracks\":[],\"user_ids\":[\"token\"]}");
        break
    }

    Ok(())
}

#[tokio::test]
async fn start_server_and_negotiate () -> Result<()> {
    prepare_test_server!();
    let (stream, _)  = prepare_test_client!();
    let(write, mut read) = stream.split();
    let write = ClientSender::new(write);

    write.send(PacketC2S::Connect {
        room_id: "1".to_string(),
        token: "token".to_string(),
    }).await?;

    while let Some(msg) = read.next().await {
        let _accept = PacketS2C::try_from(msg?).unwrap();
        break
    }

    let negotiation = PacketC2S::Negotiation(negotiation::Negotiation::SDP {
        description: webrtc::peer_connection::sdp::session_description::RTCSessionDescription::default(),
        media_type_buffer: Some(vec![])
    });
    write.send(negotiation).await?;


    while let Some(msg) = read.next().await {
        let _accept = PacketS2C::try_from(msg?).unwrap();
        break
    }

    Ok(())
}


