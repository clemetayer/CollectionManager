use super::collection_commons::{
    create_collection_from_playlist, create_new_playlist, get_playlist_id_from_url,
};
use super::handlers_models::{self, Collection, CollectionListElement};
use crate::common::common::get_env_variable;
use crate::domain;
use crate::domain::database::{
    self, domain_clear_database, get_child_collections, get_collection_id_by_deezer_id,
    remove_collection_in_database,
};
use crate::domain::deezer::add_tracks_to_playlist;
use crate::handlers::collection_commons::{convert_string_to_u64, get_playlist};
use crate::handlers::errors::*;
use crate::handlers::handlers_models::Playlist;

pub async fn init_collections(
    options: handlers_models::InitCollection,
) -> Result<usize, HandlerError> {
    // if from playlist, fill the collection with the playlist track
    match options.from_playlist {
        Some(url) => {
            return create_collection_from_playlist(get_playlist_id_from_url(url)).await;
        }
        None => {
            return create_new_playlist(options.name).await;
        }
    }
}

pub fn list_collections() -> Result<Vec<CollectionListElement>, HandlerError> {
    println!("listing collections");
    match domain::database::list_collections() {
        Ok(collections_database) => {
            let collections_handler = collections_database
                .into_iter()
                .map(|collection| {
                    let collection_element = CollectionListElement {
                        name: collection.name,
                        deezer_id: collection.deezer_id,
                        url: collection.url,
                    };
                    return collection_element;
                })
                .collect::<Vec<_>>();
            Ok(collections_handler)
        }
        Err(e) => {
            eprintln!("Error while fetching the collections from the database");
            Err(HandlerError::HandlerDatabaseError(e))
        }
    }
}

pub async fn get_collection_with_tracks(deezer_id: String) -> Result<Collection, HandlerError> {
    println!(
        "getting collection with tracks from deezer id {}",
        deezer_id.clone()
    );
    match database::get_collection_with_tracks(deezer_id.clone()) {
        Ok(collection) => {
            println!("collection = {:?}", collection);
            let playlist = get_playlist(convert_string_to_u64(&deezer_id.clone().as_str())).await?;
            let children_collections = get_direct_children_collections_without_tracks(deezer_id);
            let mut tracks = playlist.tracks;
            for children_col in children_collections.clone().into_iter() {
                let playlist = get_playlist(convert_string_to_u64(
                    &children_col.deezer_id.clone().as_str(),
                ))
                .await?;
                for track in playlist.tracks.into_iter() {
                    let track_index = tracks
                        .clone()
                        .into_iter()
                        .position(|el| el.deezer_id == track.deezer_id);
                    if track_index.is_some() {
                        tracks.remove(track_index.unwrap());
                    }
                }
            }
            return Ok(Collection {
                name: collection.name,
                deezer_id: collection.deezer_id,
                url: collection.url,
                tracks: tracks,
                children_col: children_collections,
            });
        }
        Err(e) => {
            eprintln!(
                "Error while getting the collection with tracks that has deezer id {} : {:?}",
                deezer_id, e
            );
            return Err(HandlerError::HandlerDatabaseError(e));
        }
    }
}

// will return the basic children collections (no tracks or other children collections)
fn get_direct_children_collections_without_tracks(deezer_id: String) -> Vec<Collection> {
    let mut parent_id: i32 = -1;
    let mut children_collections: Vec<Collection> = Vec::new();
    match get_collection_id_by_deezer_id(deezer_id.clone()) {
        Ok(id) => parent_id = id,
        Err(e) => {
            eprintln!(
                "Error while getting collection id from deezer_id {} : {:?}",
                deezer_id, e
            );
        }
    }
    match get_child_collections(parent_id) {
        Ok(collections) => {
            children_collections = collections
                .into_iter()
                .map(|collection| Collection {
                    name: collection.name,
                    deezer_id: collection.deezer_id,
                    url: collection.url,
                    tracks: Vec::new(),
                    children_col: Vec::new(),
                })
                .collect::<Vec<_>>();
        }
        Err(e) => {
            eprintln!(
                "Error while getting child collections of {} : {:?}",
                parent_id, e
            );
        }
    }
    return children_collections;
}

pub async fn refresh_collection_handler(collection_id: String) -> Result<bool, HandlerError> {
    println!("refreshing collection {}", collection_id.clone());
    let mut collection_id_i32: i32;
    let mut parent_playlist: Playlist;
    let mut parent_playlist_tracks_ids: Vec<String>;
    match get_collection_id_by_deezer_id(collection_id.clone()) {
        Ok(id) => collection_id_i32 = id,
        Err(e) => {
            eprintln!(
                "Error getting collection id by deezer id {} : {:?}",
                collection_id, e
            );
            return Err(HandlerError::HandlerDatabaseError(e));
        }
    }
    match get_playlist(convert_string_to_u64(&collection_id.as_str())).await {
        Ok(playlist) => {
            parent_playlist = playlist.clone();
            parent_playlist_tracks_ids = playlist
                .tracks
                .into_iter()
                .map(|track| track.deezer_id)
                .collect::<Vec<_>>()
        }
        Err(e) => {
            eprintln!("Error getting playlist {} : {:?}", collection_id, e);
            return Err(HandlerError::HandlerDeezerError());
        }
    }
    match get_child_collections(collection_id_i32) {
        Ok(child_collections) => {
            let mut tracks_to_add: Vec<String> = Vec::new();
            for collection in child_collections.into_iter() {
                println!("Adding child collection {}", collection.name);
                match get_playlist(convert_string_to_u64(&collection.deezer_id.as_str())).await {
                    Ok(child_playlist) => {
                        for track in child_playlist.tracks.into_iter() {
                            println!("Checking track {}", track.title);
                            if !tracks_to_add.contains(&track.deezer_id)
                                && !parent_playlist_tracks_ids.contains(&track.deezer_id)
                            {
                                tracks_to_add.push(track.deezer_id);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!(
                            "Error while getting child playlist {} : {:?}",
                            collection.deezer_id, e
                        );
                    }
                }
            }
            match add_tracks_to_playlist(collection_id.clone(), tracks_to_add).await {
                Ok(_) => return Ok(true),
                Err(e) => {
                    eprintln!(
                        "Error while adding tracks to the playlist {} : {:?}",
                        collection_id, e
                    );
                    return Err(HandlerError::HandlerDeezerError());
                }
            }
        }
        Err(e) => {
            eprintln!(
                "Error while getting the children from collection {} : {:?}",
                collection_id, e
            );
            return Err(HandlerError::HandlerDatabaseError(e));
        }
    }
}

pub async fn update_all_collections() -> Result<bool, HandlerError> {
    let mut playlists_ids_to_update: Vec<String> = Vec::new();
    let mut next_children_ids: Vec<String> = list_collections()?
        .into_iter()
        .map(|collection| collection.deezer_id)
        .collect::<Vec<_>>();
    for _depth in 0..get_max_depth() {
        let mut children_ids: Vec<String> = Vec::new();
        for child_id in next_children_ids.into_iter() {
            // Update the playlists ids
            if playlists_ids_to_update.contains(&child_id) {
                playlists_ids_to_update.retain(|id| *id != child_id);
            }
            playlists_ids_to_update.insert(0, child_id.clone());
            // Add the next children ids
            let mut parent_id: i32 = -1;
            match get_collection_id_by_deezer_id(child_id.clone()) {
                Ok(id) => parent_id = id,
                Err(e) => {
                    eprintln!(
                        "Error getting collection id by deezer id {} : {:?}",
                        child_id, e
                    );
                }
            }
            match get_child_collections(parent_id) {
                Ok(child_collections) => {
                    for child_collection in child_collections.into_iter() {
                        if !children_ids.contains(&child_collection.deezer_id) {
                            children_ids.push(child_collection.deezer_id);
                        }
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Error getting child collections of collection {} : {:?}",
                        child_id, e
                    );
                }
            }
        }
        // Continue or exit loop
        if children_ids.len() > 0 {
            next_children_ids = children_ids;
        } else {
            break;
        }
    }
    // Update collections
    println!("Collections to update : {:?}", playlists_ids_to_update);
    for id in playlists_ids_to_update.into_iter() {
        match refresh_collection_handler(id.clone()).await {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error while refreshing collection {} : {:?}", id, e);
            }
        }
    }
    return Ok(true);
}

fn get_max_depth() -> u64 {
    return convert_string_to_u64(&get_env_variable("MAX_COLLECTION_DEPTH").as_str());
}

pub fn remove_collection_handler(deezer_id: String) -> Result<bool, HandlerError> {
    let mut collection_id: i32 = -1;
    println!("Removing collection {}", deezer_id.clone());
    match get_collection_id_by_deezer_id(deezer_id.clone()) {
        Ok(id) => collection_id = id,
        Err(e) => {
            eprintln!(
                "Error while getting collection id from deezer_id {} : {:?}",
                collection_id, e
            );
            return Err(HandlerError::HandlerDatabaseError(e));
        }
    };
    match remove_collection_in_database(collection_id) {
        Ok(res) => return Ok(res),
        Err(e) => {
            eprintln!(
                "Error while removing collection {} in database : {:?}",
                deezer_id, e
            );
            return Err(HandlerError::HandlerDatabaseError(e));
        }
    }
}

pub fn handler_clear_database() -> Result<bool, HandlerError> {
    println!("Clearing database");
    match domain_clear_database() {
        Ok(_) => return Ok(true),
        Err(e) => return Err(HandlerError::HandlerDatabaseError(e)),
    }
}
