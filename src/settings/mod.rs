use crate::oauth::{OAuthClientId, OAuthClientSecret};
use serde::Deserialize;
use std::{fs::File, io::Read};
use url::Url;

#[derive(Deserialize)]
pub struct Settings {
    pub google_oauth: GoogleOAuthSettings,
}

pub fn get_settings() -> Settings {
    let mut settings_file = File::open("src/settings.toml").expect("Unable to open settings.toml");
    let mut toml_str = String::new();
    settings_file
        .read_to_string(&mut toml_str)
        .expect("Unable to read settings.toml");
    toml::from_str::<Settings>(&toml_str).expect("Unable to parse settings.toml")
}

#[derive(Deserialize)]
pub struct GoogleOAuthSettings {
    pub client_id: OAuthClientId,
    pub client_secret: OAuthClientSecret,
    pub redirect_url: Url,
}
