use rusqlite::{Connection, Result};

pub fn migrate(conn: &Connection) -> Result<()> {
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS favorites (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL UNIQUE,
            created_at TEXT NOT NULL
        );
        ",
        [],
    )?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS recent (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL UNIQUE,
            opened_at TEXT NOT NULL
        );
        ", [])?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT NOT NULL UNIQUE,
            value TEXT NOT NULL UNIQUE
        );
        ", [])?;

    conn.execute("
        CREATE TABLE IF NOT EXISTS tag (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            file_path TEXT NOT NULL UNIQUE,
            name TEXT NOT NULL
        );
        ", [])?;
    Ok(())
}