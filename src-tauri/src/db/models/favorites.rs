use rusqlite::Result;

use crate::db::connection;
use chrono::Local;

#[derive(Debug)]
pub struct Favorite {
    pub id: i32, 
    pub path: String,
    pub created_at: String,
}


pub fn add_favorite(path: String) -> Result<()> {
    let conn = connection::connect()?;

    conn.exec("
        INSERT INTO favorites(path, created_at)
        VALUES(?1,?2)
        ", [
            &path,
            &Local.now().to_string(),
        ])?;

    Ok(())
}

