use anyhow::Result;
use api::{
    negotiation::Negotiation,
    remote_track::RemoteTrack
};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;


/// Packet sent from the client to the server
#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum PacketC2S {
    /// Connect to a given room
    Connect {
        // Room ID
        room_id: String,
        /// Authentication token
        token: String,
    },
    /// Tell the server to send tracks
    Continue {
        /// IDs of tracks the client wants
        tracks: Vec<String>,
    },
    /// Tell the server certain tracks are no longer available
    Remove {
        /// IDs of tracks the client is no longer producing
        removed_tracks: Vec<String>,
    },
    /// Negotiation
    Negotiation(Negotiation),
}

/// Packet sent from the server to the client
#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum PacketS2C {
    /// Accept connection to room
    Accept {
        /// Currently available tracks
        available_tracks: Vec<RemoteTrack>,
        /// Users currently in the room
        user_ids: Vec<String>,
    },
    /// Tell the client about a new available track
    Announce {
        /// Newly created remote track
        track: RemoteTrack,
    },
    /// Tell the client certain tracks are no longer available
    Remove {
        /// IDs of tracks that are no longer being produced
        removed_tracks: Vec<String>,
    },
    /// Negotiation
    Negotiation(Negotiation),
    /// User joined the room
    UserJoin {
        /// ID of new user
        user_id: String,
    },
    /// User left the room
    UserLeft {
        /// ID of leaving user
        user_id: String,
    },
    /// Disconnection error
    Error { error: String },
}

impl PacketC2S {
    /// Create a packet from incoming Message
    pub fn from(message: Message) -> Result<Option<Self>> {
        Ok(if let Message::Text(text) = message {
            Some(serde_json::from_str(&text)?)
        } else {
            None
        })
    }
}
