use crate::db;
use crate::settings::get_settings;
use chrono::Utc;
use rusqlite::Connection;
use url::Url;

pub mod types;
pub use self::types::*;

pub async fn get_access_token(conn: &Connection) -> anyhow::Result<OAuthAccessToken> {
    let opt_record = db::access_token::AccessToken::get_latest(conn)?;
    match opt_record {
        Some(record) => {
            let now = Utc::now();
            if record.expires_at > now {
                Ok(record.access_token)
            } else {
                let res = crate::oauth::refresh_access_token(&record.refresh_token).await?;
                record
                    .update_access_token(conn, &res.access_token, res.expires_in)
                    .unwrap();
                Ok(res.access_token)
            }
        }
        None => {
            let res = manual_user_flow().await?;
            db::access_token::AccessToken::insert(
                conn,
                &res.access_token,
                &res.refresh_token,
                res.expires_in,
            )
            .unwrap();
            Ok(res.access_token)
        }
    }
}

pub async fn manual_user_flow() -> reqwest::Result<GetAccessTokenResponse> {
    let oauth_url = build_oauth_url().expect("Unable to build OAuth URL");

    println!("{oauth_url}");
    println!("Paste the url redirect url here:");

    let mut redirect_url_str = String::new();
    std::io::stdin().read_line(&mut redirect_url_str).unwrap();
    let redirect_url = Url::parse(redirect_url_str.trim()).unwrap();
    let redirect_params = parse_redirect_url_params(redirect_url).unwrap();
    request_access_token(redirect_params.code).await
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

pub async fn request_access_token(
    code: OAuthRedirectCode,
) -> reqwest::Result<GetAccessTokenResponse> {
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
}

pub async fn refresh_access_token(
    refresh_token: &OAuthRefreshToken,
) -> reqwest::Result<RefreshTokenResponse> {
    let settings = get_settings();
    let body = RefreshTokenParams {
        refresh_token: refresh_token.clone(),
        grant_type: "refresh_token".to_string(),
        client_id: settings.google_oauth.client_id,
        client_secret: settings.google_oauth.client_secret,
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
}
