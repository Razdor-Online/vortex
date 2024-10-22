use serde::{Deserialize, Serialize};
use webrtc::ice_transport::ice_candidate::RTCIceCandidateInit;

/// Browser compliant ICE candidate
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ICECandidate {
    pub candidate: String,
    #[serde(default)]
    pub sdp_mid: Option<String>,
    #[serde(default)]
    pub sdp_mline_index: Option<u16>,
    #[serde(default)]
    pub username_fragment: Option<String>,
}

impl From<RTCIceCandidateInit> for ICECandidate {
    fn from(candidate: RTCIceCandidateInit) -> Self {
        let RTCIceCandidateInit {
            candidate,
            sdp_mid,
            sdp_mline_index,
            username_fragment,
        } = candidate;

        Self {
            candidate,
            sdp_mid,
            sdp_mline_index,
            username_fragment,
        }
    }
}

impl From<ICECandidate> for RTCIceCandidateInit {
    fn from(candidate: ICECandidate) -> Self {
        let ICECandidate {
            candidate,
            sdp_mid,
            sdp_mline_index,
            username_fragment,
        } = candidate;

        Self {
            candidate,
            sdp_mid,
            sdp_mline_index,
            username_fragment,
        }
    }
}