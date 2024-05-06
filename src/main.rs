mod db;
mod oauth;
mod settings;

use db::get_access_token;
use oauth::OAuthAccessToken;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let access_token = match get_access_token() {
        Some(row) => OAuthAccessToken(row.access_token),
        None => {
            let token_response = oauth::run().await;
            token_response.access_token
        }
    };

    let response = get_messages(access_token).await;
    println!("{:?}", response)
}

#[derive(Deserialize, Debug)]
struct MessageId(String);

#[derive(Deserialize, Debug)]
struct ThreadId(String);

#[derive(Deserialize, Debug)]
struct MessageSnippet {
    id: MessageId,
    #[serde(rename = "threadId")]
    thread_id: ThreadId,
}
#[derive(Deserialize, Debug)]
struct GetMessagesResponse {
    messages: Vec<MessageSnippet>,
}

async fn get_messages(access_token: OAuthAccessToken) -> GetMessagesResponse {
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
