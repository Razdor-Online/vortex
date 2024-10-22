use serde::{Deserialize, Serialize};
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use crate::ice_candidate::ICECandidate;
use crate::media_type::MediaType;

/// Either description or ICE candidate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Negotiation {
    /// Session Description
    SDP {
        description: RTCSessionDescription,
        media_type_buffer: Option<Vec<MediaType>>,
    },
    /// ICE Candidate
    ICE { candidate: ICECandidate },
}