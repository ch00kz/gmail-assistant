use core::fmt;

use serde::{Deserialize, Serialize};
use url::Url;

// Google OAuth Settings
#[derive(Serialize, Deserialize)]
pub struct OAuthClientId(String);

#[derive(Serialize, Deserialize)]
pub struct OAuthClientSecret(String);

#[derive(Serialize)]
pub struct OAuthUrlParams {
    pub response_type: String,
    pub client_id: OAuthClientId,
    pub redirect_uri: Url,
    pub scope: String,
    pub access_type: String,
    pub include_granted_scopes: String,
    pub state: String,
    pub prompt: String,
}

#[derive(Serialize, Deserialize)]
pub struct OAuthRedirectCode(String);

#[derive(Deserialize)]
pub struct RedirectUrlParams {
    pub code: OAuthRedirectCode,
}

#[derive(Deserialize, Debug, Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct OAuthAccessToken(pub String);

impl fmt::Display for OAuthAccessToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Clone, sqlx::Type)]
#[sqlx(transparent)]
pub struct OAuthRefreshToken(pub String);

impl fmt::Display for OAuthRefreshToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize)]
pub struct GetAccessTokenParams {
    pub grant_type: String,
    pub client_id: OAuthClientId,
    pub client_secret: OAuthClientSecret,
    pub code: OAuthRedirectCode,
    pub redirect_uri: Url,
}

#[derive(Deserialize)]
pub struct GetAccessTokenResponse {
    pub access_token: OAuthAccessToken,
    pub refresh_token: OAuthRefreshToken,
    pub token_type: String,
    pub expires_in: u32,
    pub scope: String,
}

#[derive(Serialize)]
pub struct RefreshTokenParams {
    pub grant_type: String,
    pub client_id: OAuthClientId,
    pub client_secret: OAuthClientSecret,
    pub refresh_token: OAuthRefreshToken,
}

#[derive(Deserialize, Debug)]
pub struct RefreshTokenResponse {
    pub access_token: OAuthAccessToken,
    pub token_type: String,
    pub expires_in: u32,
    pub scope: String,
}
