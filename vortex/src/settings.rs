use config::{Config, ConfigError, Environment, File};
use serde::{Deserialize, Serialize};

pub const CONFIG_FILE_NAME: &str = "config.toml";
#[derive(Serialize, Deserialize)]
pub struct Settings {
    pub http_host: String,
    pub ws_url: String,
    pub manage_token: String,
    rtc_min_port: u16,
    rtc_max_port: u16,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name(CONFIG_FILE_NAME).required(true))
            .add_source(Environment::with_prefix("VORTEX"))
            .build()?;
        s.try_deserialize()
    }
}
