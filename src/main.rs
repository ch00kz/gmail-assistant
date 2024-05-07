mod db;
mod oauth;
mod settings;

use oauth::OAuthAccessToken;
use reqwest::Client;
use serde::Deserialize;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let conn = db::get_connection().unwrap();
    db::setup_tables(&conn).unwrap();

    let access_token = oauth::run(&conn).await;
    let response = get_messages(&access_token).await;
    let message = response.messages.get(3).unwrap();
    let response = get_message(&access_token, &message.id).await;
    println!("{:?}", response);
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

async fn get_messages(access_token: &OAuthAccessToken) -> GetMessagesResponse {
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

async fn get_message(access_token: &OAuthAccessToken, message_id: &MessageId) -> Value {
    let client = Client::new();
    let url = format!(
        "https://gmail.googleapis.com/gmail/v1/users/me/messages/{}",
        message_id.0
    );
    client
        .get(url)
        .bearer_auth(access_token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}
