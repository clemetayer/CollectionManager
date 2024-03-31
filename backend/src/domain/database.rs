use backend::models::*;
use backend::*;
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::domain::domain_models::CollectionDatabase;

use super::domain_models::InitCollectionDatabase;
use super::errors::DatabaseDomainError;

pub fn init_collection(options: InitCollectionDatabase) -> Result<usize, DatabaseDomainError> {
    match &mut establish_connection() {
        Ok(connection) => match create_collection(connection, &options.name) {
            Ok(res_size) => Ok(res_size),
            Err(e) => {
                eprintln!("Error trying to save collection : {e}");
                Err(DatabaseDomainError::ResultError(e))
            }
        },
        Err(e) => {
            eprintln!("Error trying connect to the database : {e}");
            Err(DatabaseDomainError::ConnectionError())
        }
    }
}

fn create_collection(
    conn: &mut SqliteConnection,
    name: &str,
) -> Result<usize, diesel::result::Error> {
    use backend::schema::collections;

    let new_collection = NewCollection { name };
    println!("Saving collection {name} to the database");
    diesel::insert_into(collections::table)
        .values(&new_collection)
        .execute(conn)
}

pub fn list_collections() -> Result<Vec<CollectionDatabase>, DatabaseDomainError> {
    match &mut establish_connection() {
        Ok(connection) => match load_collections(connection) {
            Ok(results) => {
                return Ok(convert_collection_list_model_to_database(results));
            }
            Err(e) => {
                eprintln!("Error trying to load the collections : {e}");
                return Err(DatabaseDomainError::ResultError(e));
            }
        },
        Err(e) => {
            eprintln!("Error trying connect to the database : {e}");
            Err(DatabaseDomainError::ConnectionError())
        }
    }
}

fn load_collections(
    connection: &mut SqliteConnection,
) -> Result<Vec<Collection>, diesel::result::Error> {
    use self::schema::collections::dsl::*;
    println!("Loading collections in the database");
    collections.select(Collection::as_select()).load(connection)
}

fn convert_collection_list_model_to_database(
    collection_model_list: Vec<Collection>,
) -> Vec<CollectionDatabase> {
    collection_model_list
        .into_iter()
        .map(|collection| convert_collection_model_to_database(collection))
        .collect::<Vec<_>>()
}

fn convert_collection_model_to_database(collection_model: Collection) -> CollectionDatabase {
    let collection_database = CollectionDatabase {
        name: collection_model.name,
    };
    return collection_database;
}
