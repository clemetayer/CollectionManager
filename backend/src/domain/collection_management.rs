use super::collection_commons::{
    create_collection_from_playlist, create_new_playlist, get_collection_id_by_deezer_id,
    get_playlist_id_from_url, log_parameters_error,
};
use super::controllers::{check_id_valid, check_init_collections};
use super::domain_models::{self, Collection, CollectionListElement, Track};
use super::errors::DomainError;
use crate::common::common::get_env_variable;
use crate::domain::collection_commons::{
    convert_string_to_u64, get_playlist, log_database_error, log_deezer_error,
};
use crate::infrastructure::database::{
    clear_database, get_child_collections, remove_collection_in_database,
};
use crate::infrastructure::deezer::add_tracks_to_playlist;
use crate::infrastructure::{self, database};
use log::error;

pub async fn init_collections(options: domain_models::InitCollection) -> Result<bool, DomainError> {
    log_parameters_error(check_init_collections(&options))?;
    // if from playlist, fill the collection with the playlist track
    match options.from_playlist {
        Some(url) => {
            return create_collection_from_playlist(&get_playlist_id_from_url(url)).await;
        }
        None => {
            return create_new_playlist(&options.name).await;
        }
    }
}

pub fn list_collections() -> Result<Vec<CollectionListElement>, DomainError> {
    match infrastructure::database::list_collections() {
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
            return Err(log_database_error(&format!(
                "Error while fetching the collections from the database : {:?}",
                e
            )));
        }
    }
}

pub async fn get_collection_tracks_excluding_children(id: &str) -> Result<Vec<Track>, DomainError> {
    log_parameters_error(check_id_valid(id.to_string()))?;
    let playlist = get_playlist(&convert_string_to_u64(id)).await?;
    let children_collections = get_direct_children_collections(id)?;
    let mut tracks = playlist.tracks;
    for children_col in children_collections.clone().into_iter() {
        let playlist =
            get_playlist(&convert_string_to_u64(children_col.deezer_id.as_str())).await?;
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
    return Ok(tracks);
}

// will return the basic children collections (no tracks or other children collections)
pub fn get_direct_children_collections(id: &str) -> Result<Vec<Collection>, DomainError> {
    log_parameters_error(check_id_valid(id.to_string()))?;
    let parent_id = get_collection_id_by_deezer_id(id)?;
    let children_collections: Vec<Collection>;
    match get_child_collections(&parent_id) {
        Ok(collections) => {
            children_collections = collections
                .into_iter()
                .map(|collection| Collection {
                    name: collection.name,
                    deezer_id: collection.deezer_id,
                    url: collection.url,
                })
                .collect::<Vec<_>>();
        }
        Err(e) => {
            return Err(log_database_error(&format!(
                "Error while getting child collections of {} : {:?}",
                parent_id, e
            )));
        }
    }
    return Ok(children_collections);
}

pub async fn get_collection(id: &str) -> Result<Collection, DomainError> {
    log_parameters_error(check_id_valid(id.to_string()))?;
    match database::get_collection(id) {
        Ok(collection) => {
            return Ok(Collection {
                name: collection.name,
                deezer_id: collection.deezer_id,
                url: collection.url,
            });
        }
        Err(e) => {
            return Err(log_database_error(&format!(
                "Error while getting the collection {} : {:?}",
                id, e
            )));
        }
    }
}

pub async fn refresh_collection(id: &str) -> Result<bool, DomainError> {
    log_parameters_error(check_id_valid(id.to_string()))?;
    let playlist = get_playlist(&convert_string_to_u64(id)).await?;
    let parent_playlist_tracks_ids = playlist
        .clone()
        .tracks
        .into_iter()
        .map(|track| track.deezer_id)
        .collect::<Vec<_>>();
    match get_child_collections(&get_collection_id_by_deezer_id(id)?) {
        Ok(child_collections) => {
            let mut tracks_to_add: Vec<String> = Vec::new();
            for collection in child_collections.into_iter() {
                let child_playlist =
                    get_playlist(&convert_string_to_u64(collection.deezer_id.as_str())).await?;
                for track in child_playlist.tracks.into_iter() {
                    if !tracks_to_add.contains(&track.deezer_id)
                        && !parent_playlist_tracks_ids.contains(&track.deezer_id)
                    {
                        tracks_to_add.push(track.deezer_id);
                    }
                }
            }
            match add_tracks_to_playlist(id, tracks_to_add).await {
                Ok(_) => return Ok(true),
                Err(e) => {
                    return Err(log_deezer_error(&format!(
                        "Error while adding tracks to the playlist {} : {:?}",
                        id, e
                    )));
                }
            }
        }
        Err(e) => {
            return Err(log_database_error(&format!(
                "Error while getting the children from collection {} : {:?}",
                id, e
            )));
        }
    }
}

pub async fn update_all_collections() -> Result<bool, DomainError> {
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
                playlists_ids_to_update.retain(|id: &String| *id != child_id);
            }
            playlists_ids_to_update.insert(0, child_id.clone());
            // Add the next children ids
            match get_child_collections(&get_collection_id_by_deezer_id(child_id.as_str())?) {
                Ok(child_collections) => {
                    for child_collection in child_collections.into_iter() {
                        if !children_ids.contains(&child_collection.deezer_id) {
                            children_ids.push(child_collection.deezer_id);
                        }
                    }
                }
                Err(e) => {
                    return Err(log_database_error(&format!(
                        "Error getting child collections of collection {} : {:?}",
                        child_id, e
                    )));
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
    for id in playlists_ids_to_update.into_iter() {
        match refresh_collection(id.as_str()).await {
            Ok(_) => {}
            Err(e) => {
                error!("Error while refreshing collection {} : {:?}", id, e);
            }
        }
    }
    return Ok(true);
}

fn get_max_depth() -> u64 {
    return convert_string_to_u64(&get_env_variable("MAX_COLLECTION_DEPTH").as_str());
}

pub fn remove_collection(id: &str) -> Result<bool, DomainError> {
    log_parameters_error(check_id_valid(id.to_string()))?;
    match remove_collection_in_database(&get_collection_id_by_deezer_id(id)?) {
        Ok(res) => return Ok(res),
        Err(e) => {
            return Err(log_database_error(&format!(
                "Error while removing collection {} in database : {:?}",
                id, e
            )));
        }
    }
}

pub fn clear_data() -> Result<bool, DomainError> {
    match clear_database() {
        Ok(_) => return Ok(true),
        Err(e) => {
            return Err(log_database_error(&format!(
                "Error while clearing the database : {:?}",
                e
            )))
        }
    }
}
