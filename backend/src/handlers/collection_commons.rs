use super::handlers_models::{Playlist, Track};
use crate::domain::database::get_collection_id_by_deezer_id;
use crate::domain::deezer::create_playlist;
use crate::domain::{self, domain_models::InitCollectionDatabase};
use crate::handlers::errors::*;
use deezer::models::{DeezerArray, PlaylistTrack};
use log::error;

pub async fn create_new_playlist(name: String) -> Result<usize, HandlerError> {
    let ret_size: usize;
    match create_playlist(name.clone()).await {
        Ok(id) => {
            let database_collection = InitCollectionDatabase {
                name: name.clone(),
                url: format!("https://www.deezer.com/fr/playlist/{}", id),
                deezer_id: format!("{}", id),
            };
            match domain::database::init_collection(database_collection) {
                Ok(size) => {
                    ret_size = size;
                }
                Err(e) => {
                    return Err(log_database_error(&format!(
                        "Error while initializing a collection {} in the database : {:?}",
                        &name, e
                    )));
                }
            };
        }
        Err(e) => {
            return Err(log_deezer_error(&format!(
                "Error creating playlist {} : {:?}",
                &name, e
            )));
        }
    }
    return Ok(ret_size);
}

pub async fn create_collection_from_playlist(playlist_id: u64) -> Result<usize, HandlerError> {
    let ret_size: usize;
    match get_playlist(playlist_id).await {
        Ok(playlist) => {
            ret_size = add_playlist_data_to_database(playlist.clone())?;
        }
        Err(e) => {
            return Err(log_deezer_error(&format!(
                "Error getting playlist {} : {:?}",
                &playlist_id, e
            )));
        }
    }
    return Ok(ret_size);
}

pub async fn get_playlist(playlist_id: u64) -> Result<Playlist, HandlerError> {
    match domain::deezer::get_playlist(playlist_id).await {
        Ok(playlist) => Ok(convert_playlist(playlist)),
        Err(e) => {
            return Err(log_deezer_error(&format!(
                "Error getting playlist {} : {:?}",
                &playlist_id, e
            )))
        }
    }
}

pub fn get_playlist_id_from_url(url: String) -> u64 {
    let id_str: Vec<&str> = url.split('/').collect();
    return convert_string_to_u64(id_str.last().unwrap());
}

pub fn convert_string_to_u64(id: &&str) -> u64 {
    match id.parse::<u64>() {
        Ok(id) => return id,
        Err(e) => {
            error!(
                "Handler : Error while converting id {} to u64 : {:?}",
                id, e
            );
            return 0;
        }
    }
}

pub fn get_track_id_from_url(url: String) -> u64 {
    let id_str: Vec<&str> = url.split('/').collect();
    match id_str.last().unwrap().parse::<u64>() {
        Ok(id) => return id,
        Err(e) => {
            error!("Handler : Error while parsing url {} : {:?}", url, e);
            return 0;
        }
    }
}

pub fn get_collection_id_by_deezer_id_handler(deezer_id: String) -> Result<i32, HandlerError> {
    match get_collection_id_by_deezer_id(deezer_id.clone()) {
        Ok(id) => {
            return Ok(id);
        }
        Err(e) => {
            return Err(log_database_error(&format!(
                "Error while getting collection id {} : {:?}",
                &deezer_id, e
            )));
        }
    };
}

pub fn log_database_error(message: &str) -> HandlerError {
    error!("Handler : {}", message);
    return HandlerError::HandlerDatabaseError();
}

pub fn log_deezer_error(message: &str) -> HandlerError {
    error!("Handler : {}", message);
    return HandlerError::HandlerDeezerError();
}

fn add_playlist_data_to_database(playlist: Playlist) -> Result<usize, HandlerError> {
    let ret_size: usize;
    let database_collection = InitCollectionDatabase {
        name: playlist.title,
        url: playlist.url.clone(),
        deezer_id: playlist.id.to_string(),
    };
    match domain::database::init_collection(database_collection) {
        Ok(size) => {
            ret_size = size;
        }
        Err(e) => {
            return Err(log_database_error(&format!(
                "Error while initializing the collection {} in the database : {:?}",
                &playlist.url, e
            )));
        }
    };
    return Ok(ret_size);
}

fn convert_playlist(playlist: deezer::models::Playlist) -> Playlist {
    return Playlist {
        id: playlist.id,
        title: playlist.title,
        public: playlist.is_public,
        nb_tracks: playlist.nb_tracks,
        url: playlist.link,
        tracks: convert_tracks(playlist.tracks),
    };
}

fn convert_tracks(tracks: DeezerArray<PlaylistTrack>) -> Vec<Track> {
    tracks
        .data
        .into_iter()
        .map(|track| convert_track(track))
        .collect::<Vec<_>>()
}

fn convert_track(track: PlaylistTrack) -> Track {
    return Track {
        id: track.id,
        title: track.title,
        link: track.link.clone(),
        artist: track.artist.name,
        deezer_id: format!("{}", get_track_id_from_url(track.link)),
    };
}
