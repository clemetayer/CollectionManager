use diesel::prelude::*;
use backend::models::*;
use backend::*;

use crate::domain::domain_models::CollectionDatabase;

use super::domain_models::InitCollectionDatabase;

pub fn init_collection(options : InitCollectionDatabase) {
    println!("Creating collection ! {:?}", &options.name);
    let connection = &mut establish_connection();
    create_collection(connection,&options.name);
}

pub fn list_collections() -> Vec<CollectionDatabase> {
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
    return results_database;
}