use crate::oauth::GetAccessTokenResponse;
use chrono::{DateTime, Duration, Utc};
use rusqlite::Connection;

pub struct AccessToken {
    id: i64,
    pub access_token: String,
    refresh_token: String,
    expires_at: DateTime<Utc>,
}

pub fn insert_access_token(response: &GetAccessTokenResponse) -> rusqlite::Result<()> {
    let conn = Connection::open("gmail-assistant.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS access_tokens (
          id              INTEGER PRIMARY KEY AUTOINCREMENT,
          access_token    TEXT NOT NULL,
          refresh_token   TEXT NOT NULL,
          expires_at      DATETIME NOT NULL
      )",
        (), // empty list of parameters.
    )?;

    let expires_at = Utc::now() + Duration::milliseconds(response.expires_in as i64);
    let record = AccessToken {
        id: 0,
        access_token: response.access_token.to_string(),
        refresh_token: response.refresh_token.to_string(),
        expires_at,
    };

    conn.execute(
        "INSERT INTO access_tokens (access_token, refresh_token, expires_at) VALUES (?1, ?2, ?3)",
        (
            &record.access_token,
            &record.refresh_token,
            &record.expires_at,
        ),
    )?;

    Ok(())
}

pub fn get_access_token() -> Option<AccessToken> {
    let now = Utc::now();
    let conn = Connection::open("gmail-assistant.db").ok()?;
    let mut stmt = conn
        .prepare("SELECT * from access_tokens where expires_at < ?")
        .ok()?;
    let mut results = stmt
        .query_map([now], |row| {
            Ok(AccessToken {
                id: row.get(0)?,
                access_token: row.get(1)?,
                refresh_token: row.get(2)?,
                expires_at: row.get(3)?,
            })
        })
        .ok()?;
    results.next().unwrap().ok()
}
