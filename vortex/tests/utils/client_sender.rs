use std::sync::{Arc};

use anyhow::Result;
use futures::{SinkExt};
use log::debug;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, MaybeTlsStream, WebSocketStream};
use signaling::packets::{PacketC2S};

type Stream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Sink side of the WebSocket stream behind a Mutex for distributed writing
#[derive(Clone)]
pub struct ClientSender {
    stream: Arc<Mutex<Stream>>,
}

impl ClientSender {
    /// Create a new Sender
    pub fn new(stream: Stream) -> Self {
        ClientSender {
            stream: Arc::new(Mutex::new(stream)),
        }
    }

    /// Send a packet through the WebSocket
    pub async fn send(&self, packet: PacketC2S) -> Result<()> {
        debug!("C->S: {:?}", packet);
        self.stream
            .lock()
            .await
            .send(Message::Text(serde_json::to_string(&packet)?))
            .await?;

        Ok(())
    }
}
