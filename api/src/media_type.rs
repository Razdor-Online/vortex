use serde::{Deserialize, Serialize};

/// Available types of media tracks
#[derive(Debug, Clone, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum MediaType {
    /// Audio stream
    Audio,
    /// Video stream
    Video,
    /// Screenshare audio stream
    ScreenAudio,
    /// Screenshare video stream
    ScreenVideo,
}

impl std::fmt::Display for MediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MediaType::Audio => write!(f, "Audio"),
            MediaType::Video => write!(f, "Video"),
            MediaType::ScreenAudio => write!(f, "ScreenAudio"),
            MediaType::ScreenVideo => write!(f, "ScreenVideo"),
        }
    }
}