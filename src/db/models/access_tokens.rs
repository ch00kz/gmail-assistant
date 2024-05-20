use chrono::{DateTime, Duration, Utc};
use sqlx::SqlitePool;

use crate::client::gmail::oauth::{OAuthAccessToken, OAuthRefreshToken};

fn to_expires_at(expires_in: u32) -> DateTime<Utc> {
    Utc::now() + Duration::milliseconds(expires_in as i64)
}

#[derive(sqlx::FromRow)]
pub struct AccessToken {
    pub id: i64,
    pub access_token: OAuthAccessToken,
    pub refresh_token: OAuthRefreshToken,
    pub expires_at: DateTime<Utc>,
}

impl AccessToken {
    pub async fn insert(
        pool: &SqlitePool,
        access_token: &OAuthAccessToken,
        refresh_token: &OAuthRefreshToken,
        expires_in: &u32,
    ) -> sqlx::Result<()> {
        sqlx::query(
            "INSERT INTO access_tokens (access_token, refresh_token, expires_at) VALUES (?1, ?2, ?3)",
        )
        .bind(access_token)
        .bind(refresh_token)
        .bind(to_expires_at(*expires_in))
        .execute(pool)
        .await?;

        Ok(())
    }

    pub async fn update_access_token(
        self,
        pool: &SqlitePool,
        access_token: &OAuthAccessToken,
        expires_in: u32,
    ) -> sqlx::Result<()> {
        let expires_at = to_expires_at(expires_in);
        sqlx::query("UPDATE access_tokens SET access_token = ?, expires_at = ? WHERE id = ?")
            .bind(access_token)
            .bind(expires_at)
            .bind(self.id)
            .execute(pool)
            .await?;

        Ok(())
    }

    pub async fn get_latest(pool: &SqlitePool) -> sqlx::Result<Option<AccessToken>> {
        let result = sqlx::query_as(
            "SELECT id, access_token, refresh_token, expires_at FROM access_tokens ORDER BY expires_at DESC LIMIT 1",
        )
        .fetch_optional(pool)
        .await?;

        Ok(result)
    }
}
