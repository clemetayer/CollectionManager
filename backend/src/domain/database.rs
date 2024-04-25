use backend::models::*;
use backend::schema::collections;
use backend::schema::tracks;
use backend::schema::tracks_in_collection;
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

pub fn add_tracks(
    tracks: Vec<handlers::handlers_models::Track>,
) -> Result<(), DatabaseDomainError> {
    match &mut establish_connection() {
        Ok(connection) => {
            println!("adding {} tracks to the database", tracks.len());
            for track in tracks.iter() {
                add_track(&convert_track_to_database(&track), connection);
            }
            return Ok(());
        }
        Err(e) => {
            eprintln!("Error trying connect to the database : {e}");
            return Err(DatabaseDomainError::ConnectionError());
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
    println!("Adding track {}", track.title);
    match diesel::insert_into(tracks::table)
        .values(track)
        .on_conflict_do_nothing()
        .execute(conn)
    {
        Ok(_) => {}
        Err(e) => {
            eprintln!("failed to add track {} : {e}", track.title);
        }
    };
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

pub fn get_track_id_by_deezer_id(deezer_id: String) -> Result<i32, DatabaseDomainError> {
    match &mut establish_connection() {
        Ok(connection) => {
            match tracks::table
                .filter(tracks::deezer_id.eq(deezer_id.clone()))
                .select(Track::as_select())
                .get_result(connection)
            {
                Ok(track) => {
                    return Ok(track.id);
                }
                Err(e) => {
                    eprintln!("Error getting the track by deezer id {deezer_id} : {e}");
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

pub fn add_track_to_collection(
    collection_id: i32,
    track_id: i32,
) -> Result<i32, DatabaseDomainError> {
    match &mut establish_connection() {
        Ok(connection) => {
            let track_in_collection = NewTrackInCollection {
                collection_id: &collection_id,
                track_id: &track_id,
            };
            match diesel::insert_into(tracks_in_collection::table)
                .values(&track_in_collection)
                .on_conflict_do_nothing()
                .execute(connection)
            {
                Ok(_) => {
                    return Ok(collection_id);
                }
                Err(e) => {
                    eprintln!(
                        "Error adding track id {track_id} to collection id {collection_id}: {e}"
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
