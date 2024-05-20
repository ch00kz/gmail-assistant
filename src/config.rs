use crate::client::gmail::oauth::{OAuthClientId, OAuthClientSecret};
use anyhow::{Context, Result};
use serde::Deserialize;
use std::{fs::File, io::Read};
use url::Url;

#[derive(Deserialize)]
pub struct Config {
    pub google_oauth: GoogleOAuthConfig,
}

pub fn get_config() -> Result<Config> {
    let mut settings_file =
        File::open("src/settings.toml").context("Unable to open settings.toml")?;
    let mut toml_str = String::new();
    settings_file
        .read_to_string(&mut toml_str)
        .context("Unable to read settings.toml")?;
    Ok(toml::from_str::<Config>(&toml_str)?)
}

#[derive(Deserialize)]
pub struct GoogleOAuthConfig {
    pub client_id: OAuthClientId,
    pub client_secret: OAuthClientSecret,
    pub redirect_url: Url,
}
