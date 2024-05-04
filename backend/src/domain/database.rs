use backend::models::*;
use backend::schema::collection_dependencies;
use backend::schema::collections;
use backend::schema::tracks;
use backend::schema::tracks_in_collection;
use backend::*;
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::domain::domain_models::CollectionDatabase;
use crate::handlers;

use super::domain_models::InitCollectionDatabase;
use super::domain_models::TrackDatabase;
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
        deezer_id: collection_model.deezer_id,
        url: collection_model.url,
        tracks: Vec::new(),
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
                match tracks_in_collection::table
                    .filter(tracks_in_collection::collection_id.eq(collection.id.clone()))
                    .select(TracksInCollection::as_select())
                    .get_results(connection)
                {
                    Ok(tracks) => {
                        let tracks_database: &mut Vec<TrackDatabase> = &mut Vec::new();
                        println!("mapping tracks in collection");
                        for track in tracks.iter() {
                            match tracks::table
                                .filter(tracks::id.eq(track.track_id.clone()))
                                .select(Track::as_select())
                                .get_result(connection)
                            {
                                Ok(track_database) => {
                                    println!(
                                        "adding track {} to map vec",
                                        track_database.deezer_id.clone()
                                    );
                                    tracks_database.push(TrackDatabase {
                                        deezer_id: track_database.deezer_id,
                                        title: track_database.title,
                                        artist: track_database.artist,
                                        url: track_database.url,
                                    });
                                }
                                Err(e) => {
                                    eprintln!("Error getting track id {} : {e}", track.track_id);
                                }
                            }
                        }
                        return Ok(CollectionDatabase {
                            deezer_id: collection.deezer_id,
                            url: collection.url,
                            name: collection.name,
                            tracks: tracks_database.clone(),
                        });
                    }
                    Err(e) => {
                        eprintln!(
                            "Error getting the tracks in collection {} : {e}",
                            deezer_id.clone()
                        );
                        return Err(DatabaseDomainError::ResultError(e));
                    }
                }
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
