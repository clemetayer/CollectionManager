use backend::models::*;
use backend::schema::tracks;
use backend::*;
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::domain::domain_models::CollectionDatabase;
use crate::handlers;

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
    use backend::schema::collections;

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

pub fn add_tracks(tracks: Vec<handlers::handlers_models::Track>) {
    match &mut establish_connection() {
        Ok(connection) => {
            tracks
                .into_iter()
                .map(|track| add_track(&convert_track_to_database(&track), connection));
        }
        Err(e) => {
            eprintln!("Error trying connect to the database : {e}");
        }
    }
}

fn convert_track_to_database(track: &handlers::handlers_models::Track) -> NewTrack {
    return NewTrack {
        title: track.title.as_str(),
        url: track.link.as_str(),
        deezer_id: track.deezer_id.as_str(),
        artist: track.artist.as_str(),
    };
}

fn add_track(track: &NewTrack, conn: &mut SqliteConnection) {
    match diesel::insert_into(tracks::table)
        .values(track)
        .on_conflict_do_nothing()
        .execute(conn)
    {
        Ok(_) => {}
        Err(e) => {
            println!("failed to add track {} : {e}", track.title);
        }
    };
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
