// use crate::models::favorite::Favorite;
// use crate::repositories::favorite_repository;
use crate::models::favorite::Favorite;
use crate::repositories::favorite_repository;
use chrono::Local;


pub fn test_repository() {
    let favorite = Favorite {
        id: 0,
        path: "/tmp/test".to_string(),
        created_at: Local::now().to_string(),
    };

    let result = favorite_repository::create(favorite);

    match result {
        Ok(_) => println!("Insert successful"),
        Err(e) => println!("Database Error: {:?}", e),
    }
}