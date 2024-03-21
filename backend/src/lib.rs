pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

use crate::models::NewCollection;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&(database_url.as_str()))
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_collection(conn: &mut SqliteConnection, name: &str) {
    use crate::schema::collections;

    let new_collection = NewCollection { name };

    diesel::insert_into(collections::table)
        .values(&new_collection)
        .execute(conn)
        .expect("Error saving new collection");
}
