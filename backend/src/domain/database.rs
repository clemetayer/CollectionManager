use diesel::prelude::*;
use backend::models::*;
use backend::*;
use warp::reply::{Reply, Response};
use std::convert::Infallible;

use crate::domain::domain_models::CollectionDatabase;

use super::domain_models::InitCollectionDatabase;

pub async fn init_collection(options : InitCollectionDatabase) -> Result<impl Reply, Infallible> {
    println!("Creating collection ! {:?}", &options.name);
    let connection = &mut establish_connection();
    create_collection(connection,&options.name);
    return Ok(warp::reply());
}

pub async fn list_collections() -> Result<impl Reply, Infallible> {
    use self::schema::collections::dsl::*;

    let connection: &mut diesel::prelude::SqliteConnection = &mut establish_connection();
    let results = collections
        .select(Collection::as_select())
        .load(connection)
        .expect("Error loading posts");
    let results_database = results
        .into_iter()
        .map(|collection| {
            let collection_database = CollectionDatabase {name: collection.name};
            collection_database
        })
        .collect::<Vec<_>>();
    return Ok(warp::reply::json(&results_database).into_response());
}