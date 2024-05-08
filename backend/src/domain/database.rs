use backend::models::*;
use backend::schema::collection_dependencies;
use backend::schema::collections;
use backend::*;
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::domain::domain_models::CollectionDatabase;

use super::domain_models::InitCollectionDatabase;
use super::errors::DatabaseDomainError;

pub fn init_collection(options: InitCollectionDatabase) -> Result<usize, DatabaseDomainError> {
    match &mut establish_connection() {
        Ok(connection) => {
            match create_collection(connection, &options.name, &options.deezer_id, &options.url) {
                Ok(res_size) => Ok(res_size),
                Err(e) => {
                    eprintln!("Error trying to save collection : {e}");
                    Err(DatabaseDomainError::ResultError(e))
                }
            }
        }
        Err(e) => {
            eprintln!("Error trying connect to the database : {e}");
            Err(DatabaseDomainError::ConnectionError())
        }
    }
}

fn create_collection(
    conn: &mut SqliteConnection,
    name: &str,
    deezer_id: &str,
    url: &str,
) -> Result<usize, diesel::result::Error> {
    let new_collection = NewCollection {
        name,
        deezer_id,
        url,
    };
    println!("Saving collection {name} to the database");
    diesel::insert_into(collections::table)
        .values(&new_collection)
        .on_conflict_do_nothing()
        .execute(conn)
}

pub fn get_collection_id_by_deezer_id(deezer_id: String) -> Result<i32, DatabaseDomainError> {
    match &mut establish_connection() {
        Ok(connection) => match collections::table
            .filter(collections::deezer_id.eq(deezer_id.clone()))
            .select(Collection::as_select())
            .get_result(connection)
        {
            Ok(collection) => {
                return Ok(collection.id);
            }
            Err(e) => {
                eprintln!("Error getting the collection by deezer id {deezer_id} : {e}");
                return Err(DatabaseDomainError::ResultError(e));
            }
        },
        Err(e) => {
            eprintln!("Error trying connect to the database : {e}");
            return Err(DatabaseDomainError::ConnectionError());
        }
    }
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
        deezer_id: collection_model.deezer_id,
        url: collection_model.url,
    };
    return collection_database;
}

pub fn get_collection_with_tracks(
    deezer_id: String,
) -> Result<CollectionDatabase, DatabaseDomainError> {
    println!(
        "getting collection with tracks from deezer id : {}",
        deezer_id.clone()
    );
    match &mut establish_connection() {
        Ok(connection) => match collections::table
            .filter(collections::deezer_id.eq(deezer_id.clone()))
            .select(Collection::as_select())
            .get_result(connection)
        {
            Ok(collection) => {
                println!("getting tracks in collection");
                return Ok(CollectionDatabase {
                    deezer_id: collection.deezer_id,
                    url: collection.url,
                    name: collection.name,
                });
            }
            Err(e) => {
                eprintln!("Error getting the collection {} : {e}", deezer_id);
                return Err(DatabaseDomainError::ResultError(e));
            }
        },
        Err(e) => {
            eprintln!("Error trying connect to the database : {e}");
            return Err(DatabaseDomainError::ConnectionError());
        }
    }
}

pub fn add_collection_to_parent(
    parent_id: i32,
    child_id: i32,
) -> Result<bool, DatabaseDomainError> {
    match &mut establish_connection() {
        Ok(connection) => {
            let collection_dependency = NewCollectionDependency {
                parent_id: &parent_id,
                child_id: &child_id,
            };
            match diesel::insert_into(collection_dependencies::table)
                .values(&collection_dependency)
                .on_conflict_do_nothing()
                .execute(connection)
            {
                Ok(_) => {
                    return Ok(true);
                }
                Err(e) => {
                    eprintln!(
                        "Error adding collection dependency : child id {child_id} to parent id {parent_id}: {e}"
                    );
                    return Err(DatabaseDomainError::ConnectionError());
                }
            }
        }
        Err(e) => {
            eprintln!("Error trying connect to the database : {e}");
            return Err(DatabaseDomainError::ConnectionError());
        }
    }
}

pub fn get_child_collections(
    parent_id: i32,
) -> Result<Vec<CollectionDatabase>, DatabaseDomainError> {
    match &mut establish_connection() {
        Ok(connection) => {
            match collection_dependencies::table
                .inner_join(
                    collections::table.on(collections::id.eq(collection_dependencies::child_id)),
                )
                .filter(collection_dependencies::parent_id.eq(parent_id))
                .select(Collection::as_select())
                .get_results(connection)
            {
                Ok(collections) => {
                    return Ok(collections
                        .into_iter()
                        .map(|collection| CollectionDatabase {
                            deezer_id: collection.deezer_id,
                            url: collection.url,
                            name: collection.name,
                        })
                        .collect::<Vec<_>>());
                }
                Err(e) => {
                    eprintln!("Error getting child collections : {e}");
                    return Err(DatabaseDomainError::ResultError(e));
                }
            }
        }
        Err(e) => {
            eprintln!("Error trying connect to the database : {e}");
            return Err(DatabaseDomainError::ConnectionError());
        }
    }
}
