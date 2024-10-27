use std::{pin::Pin};

use anyhow::Result;
use futures::{Future, StreamExt};
use log::info;
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use api::server_error::ServerError;
use super::{
    client::Client,
    packets::{PacketC2S, PacketS2C},
    server_sender::{ReadWritePair, ServerSender},
};

/// User capabilities
#[derive(Default, Debug)]
pub struct UserCapabilities {
    pub audio: bool,
    pub video: bool,
    pub screenshare: bool,
}

/// User Information
#[derive(Debug)]
pub struct UserInformation {
    pub id: String,
    pub capabilities: UserCapabilities,
}

#[allow(dead_code)]
/// Authentication function
type AuthFn = Box<
    dyn (Fn(
            String,
            String,
        ) -> Pin<Box<dyn Future<Output = Result<UserInformation>> + Send + 'static>>)
        + Send
        + Sync,
>;

/// Launch a new signaling server
pub async fn launch<A: ToSocketAddrs>(addr: A) -> Result<()> {
    // Create TCP listener
    let try_socket = TcpListener::bind(addr).await;
    let listener = try_socket.expect("Failed to bind");

    info!("Waiting for connections...");

    // Accept new connections
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream));
    }

    Ok(())
}

/// Accept a new TCP connection
async fn accept_connection(stream: TcpStream) {
    // Validate TCP connection
    stream
        .peer_addr()
        .expect("connected streams should have a peer address");

    // Handshake WebSocket connection
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");

    // Prepare the connection for read / write
    let (write, read) = ws_stream.split();
    let write = ServerSender::new(write);

    // Handle any resulting errors
    if let Err(error) = handle_connection((read, write.clone())).await {
        write
            .send(PacketS2C::Error {
                error: error.to_string(),
            })
            .await
            .ok();
    }
}

/// Wrap error handling around the connection and authenticate the client
async fn handle_connection((mut read, write): ReadWritePair) -> Result<()> {
    // Wait until valid packet is sent
    while let Some(msg) = read.next().await {
        if let Ok(packet) = PacketC2S::try_from(msg?) {
            if let PacketC2S::Connect { room_id, token } = packet {
                on_connect(room_id, token, (read, write)).await?;
                break;
            }
        }
    }
    Ok(())
}

async fn on_connect(room_id: String,
                    token: String,
                    read_write_pair: ReadWritePair) -> Result<()> {
    // Authenticate the client
    let  (read, write) = read_write_pair;
    match on_auth(room_id.to_owned(), token, write.clone()).await
    {
        Ok(user) => {
            info!("Authenticated user {} for room {room_id}", user.id);
            // Create a new client
            let client = Client::new(user, room_id);
            client.run((read, write)).await
        }
        Err(_) => {
            Err(ServerError::FailedToAuthenticate.into())
        }
    }
}

#[allow( unused_variables, dead_code)]
async fn on_auth(room_id: String, token: String, sender: ServerSender<>) -> Result<UserInformation> {
    // TODO: Implement authentication`
    Ok(UserInformation {
        id : token,
        capabilities: UserCapabilities {
            audio: true,
            video: true,
            screenshare: true,
        },
    })
}
