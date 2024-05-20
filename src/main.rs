mod client;
mod config;
mod db;

use anyhow::{anyhow, Result};
use client::gmail;

#[tokio::main]
async fn main() -> Result<()> {
    let pool = db::get_pool().await?;

    let access_token = client::gmail::oauth::get_access_token(&pool).await?;
    let response = gmail::get_messages(&access_token).await?;
    match response.messages.get(3) {
        Some(message) => {
            let response = gmail::get_message(&access_token, &message.id).await?;
            println!("{:?}", response);
            Ok(())
        }
        None => Err(anyhow!("Message not found at this index")),
    }
}
