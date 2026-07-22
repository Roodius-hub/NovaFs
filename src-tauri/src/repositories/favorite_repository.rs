


use crate::models::favorite::Favorite;
use rusqlite::{Connection, Result};
use crate::db::connection::connect;
use rusqlite::OptionalExtension;
// pub mod favorite;

// getting all favorites
pub fn get_favorites(conn: &Connection) -> Result<Vec<Favorite>> {
    let mut stmt = conn.prepare(
        "SELECT id, path, created_at FROM favorites",
    )?;

    let favorites = stmt.query_map([], |row| {
        Ok(Favorite {
            id: row.get(0)?,
            path: row.get(1)?,
            created_at: row.get(2)?,
        })
    })?;

    let mut result = Vec::new();

    for fav in  favorites {
        result.push(fav?);
    }

    Ok(result)
}

// create favorite
pub fn create(favorite: Favorite) -> Result<()> {
    let mut conn = connect()?;

    conn.execute(
        "INSERT INTO favorites(path, created_at)
             VALUES(?1, ?2)"
        , (&favorite.path, &favorite.created_at)
    )?;
    
    Ok(())
}

// get by id
pub fn get_by_id(id: i32) -> Result<Option<Favorite>> {
    let conn = connect()?;

    let favorite = conn.query_row("
        SELECT id, path,created_at FROM favorites WHERE id = ?1,
        ",[id],
        |row| {
            Ok(Favorite {
                id:row.get(0)?,
                path:row.get(1)?,
                created_at: row.get(2)?,
            })
        },
        ).optional()?;

    Ok(favorite)
}

//  update
pub fn update(favorite: Favorite) -> Result<()> {
    let conn = connect()?;

    conn.execute("
        UPDATE favorites
        SET path = ?1, WHERE id = ?2,
        ", (&favorite.path, favorite.id),)?;

    Ok(())
}

pub fn delete(id: i32) -> Result<()> {
    let conn  = connect()?;

    conn.execute("
        DELETE FROM favorites 
        WHERE  id = ?2
        ",[id])?;

    Ok(())
}