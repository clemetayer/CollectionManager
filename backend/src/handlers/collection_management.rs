use super::collection_commons::{
    create_collection_from_playlist, create_new_playlist, get_playlist_id_from_url,
};
use super::handlers_models::{self, Collection, CollectionListElement};
use crate::domain;
use crate::domain::database;
use crate::handlers::collection_commons::{convert_string_to_u64, get_playlist};
use crate::handlers::errors::*;

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
