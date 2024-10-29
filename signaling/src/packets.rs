use anyhow::Result;
use api::{negotiation::Negotiation, remote_track::RemoteTrack};
use serde::{Deserialize, Serialize};
use tokio_tungstenite::tungstenite::Message;

/// Packet sent from the client to the server

#[allow(clippy::large_enum_variant)]
#[derive(Deserialize, Serialize, Debug)]
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

impl TryFrom<Message> for PacketC2S {
    //TODO Specify own error instead of string
    type Error = &'static str;

    /// Try to create a packet from incoming Message
    fn try_from(value: Message) -> Result<Self, Self::Error> {
        if let Message::Text(text) = value {
            if let Ok(packet) = serde_json::from_str(&text) {
                return Ok(packet);
            }
        }
        Err("PacketC2S must be Message::Text")
    }
}

/// Packet sent from the server to the client
#[allow(clippy::large_enum_variant)]
#[derive(Deserialize, Serialize, Debug)]
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

impl PacketS2C {
    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl TryFrom<Message> for PacketS2C {
    //TODO Specify own error instead of string
    type Error = &'static str;

    /// Try to create a packet from incoming Message
    fn try_from(
        value: Message,
    ) -> std::result::Result<Self, <PacketS2C as TryFrom<Message>>::Error> {
        if let Message::Text(text) = value {
            if let Ok(packet) = serde_json::from_str(&text) {
                return Ok(packet);
            }
        }
        Err("PacketS2C must be Message::Text")
    }
}
