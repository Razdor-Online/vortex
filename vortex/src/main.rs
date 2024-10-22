#[macro_use]
extern crate log;
extern crate serde;
extern crate lazy_static;

use anyhow::Result;
use api::server_error::ServerError;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or("RUST_LOG", "info"));

    let address = dotenv::var("HTTP_HOST")
        .unwrap_or_else(|_| "127.0.0.1:3000".to_string());

    info!("Starting vortex server at {}", address);

    signaling::server::launch(
        address,
        Box::new(move |room_id, token| {
            Box::pin(async move {
                if room_id != "1" {
                    return Err(ServerError::RoomNotFound.into());
                }

                let id = token.to_string();

                use signaling::server::{UserCapabilities, UserInformation};
                Ok(UserInformation {
                    id,
                    capabilities: UserCapabilities {
                        audio: true,
                        video: true,
                        screenshare: true,
                    },
                })
            })
        }),
    )
    .await
}
