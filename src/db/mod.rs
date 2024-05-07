use rusqlite::Connection;

use self::access_token::AccessToken;

pub mod access_token;

pub fn get_connection() -> rusqlite::Result<Connection> {
    Connection::open("gmail-assistant.db")
}

pub fn setup_tables(conn: &Connection) -> rusqlite::Result<()> {
    AccessToken::create_table(conn)
}
