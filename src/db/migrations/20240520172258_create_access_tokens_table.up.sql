create table if not exists "access_tokens" (
  "id" integer primary key autoincrement,
  "access_token" text not null,
  "refresh_token" text not null,
  "expires_at" datetime not null
)
