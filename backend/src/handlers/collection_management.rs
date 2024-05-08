use super::collection_commons::{
    create_collection_from_playlist, create_new_playlist, get_playlist_id_from_url,
};
use super::handlers_models::{self, Collection, CollectionListElement};
use crate::domain;
use crate::domain::database::{self, get_child_collections, get_collection_id_by_deezer_id};
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
            return Ok(Collection {
                name: collection.name,
                deezer_id: collection.deezer_id,
                url: collection.url,
                tracks: playlist.tracks,
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
