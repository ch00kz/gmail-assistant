use crate::config;
use crate::db;
use anyhow::{anyhow, Result};
use chrono::Utc;
use db::models::access_tokens::AccessToken;
use sqlx::SqlitePool;
use url::Url;

pub mod types;
pub use self::types::*;

pub async fn get_access_token(pool: &SqlitePool) -> Result<OAuthAccessToken> {
    let opt_record: Option<AccessToken> = AccessToken::get_latest(pool).await?;
    match opt_record {
        Some(record) => {
            let now = Utc::now();
            if record.expires_at > now {
                Ok(record.access_token)
            } else {
                let res = refresh_access_token(&record.refresh_token).await?;
                record
                    .update_access_token(pool, &res.access_token, res.expires_in)
                    .await?;
                Ok(res.access_token)
            }
        }
        None => {
            let res = manual_user_flow().await?;
            AccessToken::insert(pool, &res.access_token, &res.refresh_token, &res.expires_in)
                .await?;
            Ok(res.access_token)
        }
    }
}

pub async fn manual_user_flow() -> Result<GetAccessTokenResponse> {
    let oauth_url = build_oauth_url()?;

    println!("{oauth_url}");
    println!("Paste the url redirect url here:");

    let mut redirect_url_str = String::new();
    std::io::stdin().read_line(&mut redirect_url_str)?;
    let redirect_url = Url::parse(redirect_url_str.trim())?;
    let redirect_params = parse_redirect_url_params(redirect_url)?;
    request_access_token(redirect_params.code).await
}

fn build_oauth_url() -> Result<Url> {
    let config = config::get_config()?;
    let params = OAuthUrlParams {
        response_type: "code".to_string(),
        client_id: config.google_oauth.client_id,
        redirect_uri: config.google_oauth.redirect_url,
        scope: "https://www.googleapis.com/auth/gmail.readonly".to_string(),
        access_type: "offline".to_string(),
        include_granted_scopes: "true".to_string(),
        state: "state_parameter_passthrough_value".to_string(),
        prompt: "consent".to_string(),
    };
    let query = serde_urlencoded::to_string(params)?;
    let url_str = format!("https://accounts.google.com/o/oauth2/v2/auth?{query}");
    Ok(Url::parse(&url_str)?)
}

fn parse_redirect_url_params(redirect_url: Url) -> Result<RedirectUrlParams> {
    let query_str = redirect_url
        .query()
        .ok_or(anyhow!("Unable to extract query from {redirect_url}"))?;
    Ok(serde_urlencoded::from_str(query_str)?)
}

pub async fn request_access_token(code: OAuthRedirectCode) -> Result<GetAccessTokenResponse> {
    let config = config::get_config()?;
    let body = GetAccessTokenParams {
        code,
        grant_type: "authorization_code".to_string(),
        client_id: config.google_oauth.client_id,
        client_secret: config.google_oauth.client_secret,
        redirect_uri: config.google_oauth.redirect_url,
    };
    let client = reqwest::Client::new();
    Ok(client
        .post("https://oauth2.googleapis.com/token")
        .form(&body)
        .send()
        .await?
        .json()
        .await?)
}

pub async fn refresh_access_token(
    refresh_token: &OAuthRefreshToken,
) -> Result<RefreshTokenResponse> {
    let config = config::get_config()?;
    let body = RefreshTokenParams {
        refresh_token: refresh_token.clone(),
        grant_type: "refresh_token".to_string(),
        client_id: config.google_oauth.client_id,
        client_secret: config.google_oauth.client_secret,
    };
    let client = reqwest::Client::new();
    Ok(client
        .post("https://oauth2.googleapis.com/token")
        .form(&body)
        .send()
        .await?
        .json()
        .await?)
}
