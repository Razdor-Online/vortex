use crate::media_type::MediaType;
use serde::{Deserialize, Serialize};

/// Representation of an available track on the server
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RemoteTrack {
    /// ID of the track
    pub id: String,
    /// User ID of whoever owns the track
    pub user_id: String,
    /// Type of media this track provides
    pub media_type: MediaType,
}
