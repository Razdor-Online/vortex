use std::sync::Arc;

use anyhow::Result;
use futures::{stream::{SplitSink, SplitStream}, SinkExt};
use log::debug;
use tokio::{net::TcpStream, sync::Mutex};
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
use super::packets::PacketS2C;


type Sink = SplitSink<WebSocketStream<TcpStream>, Message>;

/// Sink side of the WebSocket stream behind a Mutex for distributed writing
#[derive(Clone)]
pub struct ServerSender {
    write: Arc<Mutex<Sink>>,
}

impl ServerSender {
    /// Create a new Sender
    pub fn new(sink: Sink) -> Self {
        ServerSender {
            write: Arc::new(Mutex::new(sink)),
        }
    }

    /// Send a packet through the WebSocket
    pub async fn send(&self, packet: PacketS2C) -> Result<()> {
        debug!("S->C: {:?}", packet);
        self.write
            .lock()
            .await
            .send(Message::Text(serde_json::to_string(&packet)?))
            .await?;

        Ok(())
    }
}

/// Pair of sink and stream

pub type ReadWritePair = (SplitStream<WebSocketStream<TcpStream>>, ServerSender);

