use rusqlite::{params, Connection, Result};

use super::User;

fn open_db() -> Result<Connection> {
    let conn = Connection::open("acfun-live-toobox.db")?;

    Ok(conn)
}

pub fn initialize() -> Result<Connection> {
    let conn = open_db()?;

    conn.execute(
        "create table if not exists kv(key text unique, value text)",
        [],
    )?;

    Ok(conn)
}

pub fn load_user(conn: Connection) -> Result<Option<User>> {
    let value = conn.query_row("SELECT value FROM kv WHERE key = 'user'", [], |row| {
        row.get(0)
    })?;

    if let Err((_c, e)) = conn.close() {
        log::log!(
            log::Level::Error,
            "Connection failed to close with error {:?}",
            e
        )
    }

    let user = serde_json::from_value(value);
    match user {
        Ok(u) => Ok(u),
        Err(e) => {
            log::log!(
                log::Level::Error,
                "serde_json::from_value for User with error {:?}",
                e
            );
            Ok(None)
        }
    }
}

pub fn save_user(user: &User) -> Result<()> {
    let conn = open_db()?;

    conn.execute(
        "INSERT INTO kv (key, value) VALUES (?, ?)",
        params!["user", serde_json::to_value(user).unwrap()],
    )?;

    if let Err((_c, e)) = conn.close() {
        log::log!(
            log::Level::Error,
            "Connection failed to close with error {:?}",
            e
        )
    }

    Ok(())
}
