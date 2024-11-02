mod settings;

#[macro_use]
extern crate log;
extern crate lazy_static;
extern crate serde;

use crate::settings::Settings;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().filter_or("RUST_LOG", "info"));

    let settings = Settings::new()?;

    info!("Starting vortex server at {}", &settings.http_host);

    signaling::server::launch(&settings.http_host).await
}
