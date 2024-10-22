use thiserror::Error;

/// An error occurred on the server
#[derive(Error, Debug)]
pub enum ServerError {
    #[error("This room ID does not exist.")]
    RoomNotFound,
    #[error("This track ID does not exist.")]
    TrackNotFound,
    #[error("Something went wrong trying to authenticate you.")]
    FailedToAuthenticate,
    #[error("Already connected to a room!")]
    AlreadyConnected,
    #[error("Not connected to any room!")]
    NotConnected,
    #[error("Media type already has an existing track!")]
    MediaTypeSatisfied,
}