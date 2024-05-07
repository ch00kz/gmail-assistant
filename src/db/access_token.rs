use chrono::{DateTime, Duration, Utc};
use rusqlite::Connection;

use crate::oauth::{OAuthAccessToken, OAuthRefreshToken};

fn to_expires_at(expires_in: u32) -> DateTime<Utc> {
    Utc::now() + Duration::milliseconds(expires_in as i64)
}

pub struct AccessToken {
    pub id: Option<i64>,
    pub access_token: OAuthAccessToken,
    pub refresh_token: OAuthRefreshToken,
    pub expires_at: DateTime<Utc>,
}

impl AccessToken {
    pub fn create_table(conn: &Connection) -> rusqlite::Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS access_tokens (
                id              INTEGER PRIMARY KEY AUTOINCREMENT,
                access_token    TEXT NOT NULL,
                refresh_token   TEXT NOT NULL,
                expires_at      DATETIME NOT NULL
             )",
            (), // empty list of parameters.
        )?;
        Ok(())
    }

    pub fn insert(
        conn: &Connection,
        access_token: &OAuthAccessToken,
        refresh_token: &OAuthRefreshToken,
        expires_in: u32,
    ) -> rusqlite::Result<()> {
        conn.execute(
            "INSERT INTO access_tokens (access_token, refresh_token, expires_at) VALUES (?1, ?2, ?3)",
            (
                access_token,
                refresh_token,
                to_expires_at(expires_in),
            ),
        )?;

        Ok(())
    }

    pub fn update_access_token(
        self,
        conn: &Connection,
        access_token: &OAuthAccessToken,
        expires_in: u32,
    ) -> rusqlite::Result<()> {
        conn.execute(
            "UPDATE access_tokens SET access_token = ?1, expires_at = ?2 WHERE id = ?3",
            (access_token, to_expires_at(expires_in), self.id),
        )?;
        Ok(())
    }

    pub fn get_latest(conn: &Connection) -> Option<AccessToken> {
        let mut stmt = conn
            .prepare("SELECT * FROM access_tokens ORDER BY expires_at DESC LIMIT 1")
            .ok()?;
        let mut results = stmt
            .query_map([], |row| {
                Ok(AccessToken {
                    id: row.get(0)?,
                    access_token: row.get(1)?,
                    refresh_token: row.get(2)?,
                    expires_at: row.get(3)?,
                })
            })
            .ok()?;
        results.next().map_or(None, |f| f.ok())
    }
}
