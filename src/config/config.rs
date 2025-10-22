use color_eyre::Result;
use figment::{Figment, providers::Env};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub server_host: String,
    pub server_port: u32,
}

pub fn get_config() -> Result<Config> {
    let config: Config = Figment::new()
        .merge(Env::prefixed("EXAMPLE_APP_"))
        .extract()?;

    Ok(config)
}
