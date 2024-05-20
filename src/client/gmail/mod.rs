use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

use self::oauth::OAuthAccessToken;

pub mod oauth;

#[derive(Deserialize, Debug)]
pub struct MessageId(pub String);

#[derive(Deserialize, Debug)]
pub struct MessageSnippet {
    pub id: MessageId,
}
#[derive(Deserialize, Debug)]
pub struct GetMessagesResponse {
    pub messages: Vec<MessageSnippet>,
}

pub async fn get_messages(access_token: &OAuthAccessToken) -> reqwest::Result<GetMessagesResponse> {
    let client = Client::new();
    client
        .get("https://gmail.googleapis.com/gmail/v1/users/me/messages")
        .bearer_auth(access_token)
        .send()
        .await?
        .json()
        .await
}

pub async fn get_message(
    access_token: &OAuthAccessToken,
    message_id: &MessageId,
) -> reqwest::Result<Value> {
    let client = Client::new();
    let url = format!(
        "https://gmail.googleapis.com/gmail/v1/users/me/messages/{}",
        message_id.0
    );
    client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await?
        .json()
        .await
}
