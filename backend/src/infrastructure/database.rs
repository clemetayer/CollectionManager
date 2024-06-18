use crate::common::common::get_env_variable;
use crate::infrastructure::database_models::CollectionDatabase;
use backend::models::*;
use backend::schema::collection_dependencies;
use backend::schema::collections;
use diesel::prelude::*;
use diesel::SqliteConnection;
use log::error;
use log::info;

use super::database_models::InitCollectionDatabase;
use super::errors::DatabaseError;

pub fn init_collection(options: InitCollectionDatabase) -> Result<usize, DatabaseError> {
    info!(
        "Database : initializing collection {} to database",
        &options.url
    );
    match create_collection(
        &mut get_connection()?,
        &options.name,
        &options.deezer_id,
        &options.url,
    ) {
        Ok(res_size) => Ok(res_size),
        Err(e) => {
            return Err(log_result_error(&format!(
                "Error trying to save collection {} : {:?}",
                &options.url, e
            )));
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
    info!("Database : saving collection {} to database", &url);
    diesel::insert_into(collections::table)
        .values(&new_collection)
        .on_conflict_do_nothing()
        .execute(conn)
}

pub fn get_collection_id_by_deezer_id(deezer_id: String) -> Result<i32, DatabaseError> {
    info!("Database : getting collection by deezer id {}", deezer_id);
    match collections::table
        .filter(collections::deezer_id.eq(deezer_id.clone()))
        .select(Collection::as_select())
        .get_result(&mut get_connection()?)
    {
        Ok(collection) => {
            return Ok(collection.id);
        }
        Err(e) => {
            return Err(log_result_error(&format!(
                "Error getting the collection by deezer id {} : {:?}",
                &deezer_id, e
            )));
        }
    }
}

pub fn list_collections() -> Result<Vec<CollectionDatabase>, DatabaseError> {
    info!("Database : listing collections");
    match load_collections(&mut get_connection()?) {
        Ok(results) => {
            return Ok(convert_collection_list_model_to_database(results));
        }
        Err(e) => {
            return Err(log_result_error(&format!(
                "Error trying to load the collections : {:?}",
                e
            )));
        }
    }
}

pub fn get_collection_with_tracks(deezer_id: String) -> Result<CollectionDatabase, DatabaseError> {
    info!(
        "Database : getting collection with tracks from deezer id : {}",
        &deezer_id
    );
    match collections::table
        .filter(collections::deezer_id.eq(deezer_id.clone()))
        .select(Collection::as_select())
        .get_result(&mut get_connection()?)
    {
        Ok(collection) => {
            return Ok(CollectionDatabase {
                deezer_id: collection.deezer_id,
                url: collection.url,
                name: collection.name,
            });
        }
        Err(e) => {
            return Err(log_result_error(&format!(
                "Error getting the collection {} : {:?}",
                &deezer_id, e
            )));
        }
    }
}

pub fn add_collection_to_parent(parent_id: i32, child_id: i32) -> Result<bool, DatabaseError> {
    info!(
        "Database : adding collection {} to {}",
        &child_id, &parent_id
    );
    let collection_dependency = NewCollectionDependency {
        parent_id: &parent_id,
        child_id: &child_id,
    };
    match diesel::insert_into(collection_dependencies::table)
        .values(&collection_dependency)
        .on_conflict_do_nothing()
        .execute(&mut get_connection()?)
    {
        Ok(_) => {
            return Ok(true);
        }
        Err(e) => {
            return Err(log_result_error(&format!(
                "Error adding collection dependency : child id {} to parent id {}: {:?}",
                &child_id, &parent_id, e
            )));
        }
    }
}

pub fn get_child_collections(parent_id: i32) -> Result<Vec<CollectionDatabase>, DatabaseError> {
    info!("Database : getting child collections of {}", &parent_id);
    match collection_dependencies::table
        .inner_join(collections::table.on(collections::id.eq(collection_dependencies::child_id)))
        .filter(collection_dependencies::parent_id.eq(parent_id))
        .select(Collection::as_select())
        .get_results(&mut get_connection()?)
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
            return Err(log_result_error(&format!(
                "Error getting child collections of {} : {:?}",
                &parent_id, e
            )));
        }
    }
}

pub fn remove_collection_to_parent(parent_id: i32, child_id: i32) -> Result<bool, DatabaseError> {
    info!(
        "Database : removing child collection {} from {}",
        &child_id, &parent_id
    );
    match diesel::delete(
        collection_dependencies::table.filter(
            collection_dependencies::parent_id
                .eq(parent_id)
                .and(collection_dependencies::child_id.eq(child_id)),
        ),
    )
    .execute(&mut get_connection()?)
    {
        Ok(_) => return Ok(true),
        Err(e) => {
            return Err(log_result_error(&format!(
                "Error removing child collection {} from {} : {:?}",
                &child_id, &parent_id, e
            )));
        }
    }
}

pub fn remove_collection_in_database(collection_id: i32) -> Result<bool, DatabaseError> {
    let mut res = true;
    let connection = &mut get_connection()?;
    info!(
        "Database : removing collection {} from the database",
        &collection_id
    );
    match diesel::delete(
        collection_dependencies::table.filter(collection_dependencies::parent_id.eq(collection_id)),
    )
    .execute(connection)
    {
        Ok(_) => res &= true,
        Err(e) => {
            return Err(log_result_error(&format!(
                "Error removing collection dependencies of {} from the database {:?}",
                &collection_id, e,
            )));
        }
    };
    match diesel::delete(
        collection_dependencies::table.filter(collection_dependencies::child_id.eq(collection_id)),
    )
    .execute(connection)
    {
        Ok(_) => res &= true,
        Err(e) => {
            return Err(log_result_error(&format!(
                "Error removing collection dependencies of {} from the database {:?}",
                &collection_id, e,
            )));
        }
    };
    match diesel::delete(collections::table.filter(collections::id.eq(collection_id)))
        .execute(connection)
    {
        Ok(_) => res &= true,
        Err(e) => {
            return Err(log_result_error(&format!(
                "Error removing collection {} from the database {:?}",
                &collection_id, e
            )));
        }
    };
    return Ok(res);
}

pub fn domain_clear_database() -> Result<bool, DatabaseError> {
    info!("Database : clearing the database");
    let connection = &mut get_connection()?;
    match diesel::delete(collections::table).execute(connection) {
        Ok(_) => {
            info!("Database : collections cleared");
        }
        Err(e) => {
            return Err(log_result_error(&format!(
                "Error clearing collections from the database : {:?}",
                e
            )));
        }
    };
    match diesel::delete(collection_dependencies::table).execute(connection) {
        Ok(_) => {
            info!("Database : collection dependencies cleared");
        }
        Err(e) => {
            return Err(log_result_error(&format!(
                "Error clearing collections dependencies from the database : {:?}",
                e
            )));
        }
    };
    return Ok(true);
}

fn establish_connection() -> Result<SqliteConnection, ConnectionError> {
    let database_url: String = get_env_variable("DATABASE_URL");
    SqliteConnection::establish(&(database_url.as_str()))
}

fn get_connection() -> Result<SqliteConnection, DatabaseError> {
    info!("Database : getting connection to database");
    match establish_connection() {
        Ok(conn) => return Ok(conn),
        Err(_) => {
            return Err(log_connection_error());
        }
    }
}

fn load_collections(
    connection: &mut SqliteConnection,
) -> Result<Vec<Collection>, diesel::result::Error> {
    use backend::schema::collections::dsl::*;
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

fn log_connection_error() -> DatabaseError {
    error!("Database : Error trying connect to the database");
    return DatabaseError::ConnectionError();
}

fn log_result_error(message: &str) -> DatabaseError {
    error!("Handler : {}", message);
    return DatabaseError::ResultError();
}
