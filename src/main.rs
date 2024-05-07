mod db;
mod oauth;
mod settings;

use anyhow::{anyhow, Result};
use oauth::OAuthAccessToken;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<()> {
    let conn = db::get_connection()?;
    db::setup_tables(&conn)?;

    let access_token = oauth::get_access_token(&conn).await?;
    let response = get_messages(&access_token).await?;
    match response.messages.get(3) {
        Some(message) => {
            let response = get_message(&access_token, &message.id).await?;
            println!("{:?}", response);
            Ok(())
        }
        None => Err(anyhow!("Message not found at this index")),
    }
}

#[derive(Deserialize, Debug)]
struct MessageId(pub String);

#[derive(Deserialize, Debug)]
struct MessageSnippet {
    id: MessageId,
    // #[serde(rename = "threadId")]
    // thread_id: ThreadId,
}
#[derive(Deserialize, Debug)]
struct GetMessagesResponse {
    messages: Vec<MessageSnippet>,
}

async fn get_messages(access_token: &OAuthAccessToken) -> reqwest::Result<GetMessagesResponse> {
    let client = Client::new();
    client
        .get("https://gmail.googleapis.com/gmail/v1/users/me/messages")
        .bearer_auth(access_token)
        .send()
        .await?
        .json()
        .await
}

async fn get_message(
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
