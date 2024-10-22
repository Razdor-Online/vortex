use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use dashmap::DashMap;
use log::{debug, info};
use tokio::sync::Mutex;
use webrtc::peer_connection::RTCPeerConnection;
use ::api::media_type::MediaType;
use ::api::server_error::ServerError;

use self::{
    monitor::Monitor,
    negotiation::{NegotiationFn, NegotiationState},
};

use super::room::Room;

mod api;
mod events;
mod monitor;
mod negotiation;

/// Map of media types to track IDs
pub type PeerTrackMap = Arc<Mutex<HashMap<MediaType, String>>>;

/// Abstraction of a WebRTC peer connection
#[derive(Clone)]
pub struct Peer {
    room: Arc<Room>,
    user_id: String,
    connection: Arc<RTCPeerConnection>,
    track_map: PeerTrackMap,
    track_list: Arc<DashMap<String, Monitor>>,
    negotiation_state: Arc<NegotiationState>,
    negotiation_fn: Arc<NegotiationFn>,
    media_type_buffer: Arc<Mutex<Vec<MediaType>>>,
}

impl Peer {
    /// Create a new Peer
    pub async fn new(
        user_id: String,
        room: Arc<Room>,
        negotiation_fn: NegotiationFn,
    ) -> Result<Self> {
        // Create a new RTCPeerConnection
        let connection = api::create_peer_connection().await?;

        // Create and share track map
        let track_map: PeerTrackMap = Default::default();
        room.join_user(user_id.to_owned(), track_map.clone());

        // Construct new Peer
        let peer = Self {
            room,
            user_id,
            connection,
            track_map,
            track_list: Default::default(),
            negotiation_state: Default::default(),
            negotiation_fn: Arc::new(negotiation_fn),
            media_type_buffer: Default::default(),
        };

        // Register event handlers
        peer.register_handlers().await;

        Ok(peer)
    }

    /// Clean up any open connections
    pub async fn clean_up(&self) -> Result<()> {
        // TODO: find out if tracks are removed too
        self.connection.close().await.map_err(Into::into)
    }

    /// Start reading from an existing track in the room
    pub async fn add_track(&self, id: String) -> Result<()> {
        info!("Peer is about to start reading {id}");
        if let Some(local_track) = self.room.get_track(&id) {
            let monitor = Monitor::from(&self.connection, local_track).await?;
            self.track_list.insert(id, monitor);

            Ok(())
        } else {
            Err(ServerError::TrackNotFound.into())
        }
    }

    /// Stop reading from a track
    pub async fn remove_track(&self, id: &str) -> Result<()> {
        info!("Peer is about to stop reading {id}");
        if let Some((_, monitor)) = self.track_list.remove(id) {
            debug!("Found a monitor and removing it");
            self.connection.remove_track(&monitor.close()).await?;
        }

        Ok(())
    }
}
