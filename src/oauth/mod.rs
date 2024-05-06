use core::fmt;

use crate::db::insert_access_token;
use crate::settings::get_settings;
use serde::{Deserialize, Serialize};
use url::Url;

pub async fn run() -> GetAccessTokenResponse {
    let oauth_url = build_oauth_url().expect("Unable to build OAuth URL");

    println!("{oauth_url}");
    println!("Paste the url redirect url here:");

    let mut redirect_url_str = String::new();
    std::io::stdin().read_line(&mut redirect_url_str).unwrap();
    let redirect_url = Url::parse(redirect_url_str.trim()).unwrap();
    let redirect_params = parse_redirect_url_params(redirect_url).unwrap();
    let response = get_access_token(redirect_params.code).await;
    insert_access_token(&response).unwrap();
    response
}

// Google OAuth Settings
#[derive(Serialize, Deserialize)]
pub struct OAuthClientId(String);

#[derive(Serialize, Deserialize)]
pub struct OAuthClientSecret(String);

#[derive(Serialize)]
struct OAuthUrlParams {
    response_type: String,
    client_id: OAuthClientId,
    redirect_uri: Url,
    scope: String,
    access_type: String,
    include_granted_scopes: String,
    state: String,
    prompt: String,
}

#[derive(Serialize, Deserialize)]
pub struct OAuthRedirectCode(String);

#[derive(Deserialize)]
struct RedirectUrlParams {
    code: OAuthRedirectCode,
}

fn build_oauth_url() -> Option<Url> {
    let settings = get_settings();
    let params = OAuthUrlParams {
        response_type: "code".to_string(),
        client_id: settings.google_oauth.client_id,
        redirect_uri: settings.google_oauth.redirect_url,
        scope: "https://www.googleapis.com/auth/gmail.readonly".to_string(),
        access_type: "offline".to_string(),
        include_granted_scopes: "true".to_string(),
        state: "state_parameter_passthrough_value".to_string(),
        prompt: "consent".to_string(),
    };
    let query = serde_urlencoded::to_string(params).ok()?;
    let url_str = format!("https://accounts.google.com/o/oauth2/v2/auth?{query}");
    Url::parse(&url_str).ok()
}

fn parse_redirect_url_params(redirect_url: Url) -> Option<RedirectUrlParams> {
    let query_str = redirect_url.query()?;
    serde_urlencoded::from_str(query_str).ok()
}

#[derive(Deserialize)]
pub struct OAuthAccessToken(pub String);

impl fmt::Display for OAuthAccessToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Deserialize)]
pub struct OAuthRefreshToken(String);

impl fmt::Display for OAuthRefreshToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
#[derive(Serialize)]
pub struct GetAccessTokenParams {
    grant_type: String,
    client_id: OAuthClientId,
    client_secret: OAuthClientSecret,
    code: OAuthRedirectCode,
    redirect_uri: Url,
}

#[derive(Deserialize)]
pub struct GetAccessTokenResponse {
    pub access_token: OAuthAccessToken,
    pub refresh_token: OAuthRefreshToken,
    pub token_type: String,
    pub expires_in: u32,
    pub scope: String,
}

pub async fn get_access_token(code: OAuthRedirectCode) -> GetAccessTokenResponse {
    let settings = get_settings();
    let body = GetAccessTokenParams {
        code,
        grant_type: "authorization_code".to_string(),
        client_id: settings.google_oauth.client_id,
        client_secret: settings.google_oauth.client_secret,
        redirect_uri: settings.google_oauth.redirect_url,
    };
    let client = reqwest::Client::new();
    client
        .post("https://oauth2.googleapis.com/token")
        .form(&body)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
