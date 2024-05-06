mod oauth;
mod settings;
use oauth::OAuthAccessToken;
use reqwest::Client;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let token_response = oauth::run().await;
    let response = get_messages(token_response.access_token).await;
    println!("{:?}", response)
}

async fn get_messages(access_token: OAuthAccessToken) -> Value {
    let client = Client::new();
    client
        .get("https://gmail.googleapis.com/gmail/v1/users/me/messages")
        .bearer_auth(access_token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
