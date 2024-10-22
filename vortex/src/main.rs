mod settings;

#[macro_use]
extern crate log;
extern crate serde;
extern crate lazy_static;

use anyhow::Result;
use api::server_error::ServerError;
use crate::settings::Settings;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default()
        .filter_or("RUST_LOG", "info"));

    let settings = Settings::new()?;

    info!("Starting vortex server at {}", &settings.http_host);

    signaling::server::launch(
        &settings.http_host,
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
