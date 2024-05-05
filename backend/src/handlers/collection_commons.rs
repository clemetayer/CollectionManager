use super::handlers_models::{Playlist, Track};
use crate::domain::deezer::create_playlist;
use crate::domain::{self, domain_models::InitCollectionDatabase};
use crate::handlers::errors::*;
use deezer::models::{DeezerArray, PlaylistTrack};

pub async fn create_new_playlist(name: String) -> Result<usize, HandlerError> {
    let ret_size: usize;
    println!("Creating new collection in deezer");
    match create_playlist(name.clone()).await {
        Ok(id) => {
            let database_collection = InitCollectionDatabase {
                name: name.clone(),
                url: format!("https://www.deezer.com/fr/playlist/{}", id),
                deezer_id: format!("{}", id),
            };
            println!("Initializing collection");
            match domain::database::init_collection(database_collection) {
                Ok(size) => {
                    ret_size = size;
                }
                Err(e) => {
                    eprintln!("Error while initializing a collection in the database");
                    return Err(HandlerError::HandlerDatabaseError(e));
                }
            };
        }
        Err(e) => {
            eprintln!("Error creating playlist {} : {}", name.clone(), e);
            return Err(HandlerError::HandlerDeezerError());
        }
    }
    return Ok(ret_size);
}

pub async fn create_collection_from_playlist(playlist_id: u64) -> Result<usize, HandlerError> {
    let ret_size: usize;
    println!("Creating collection from deezer playlist");
    match get_playlist(playlist_id).await {
        Ok(playlist) => {
            ret_size = add_playlist_data_to_database(playlist.clone())?;
            add_tracks_to_collection(playlist);
        }
        Err(e) => {
            eprintln!("Error getting playlist {} : {:?}", playlist_id.clone(), e);
            return Err(HandlerError::HandlerDeezerError());
        }
    }
    return Ok(ret_size);
}

fn add_playlist_data_to_database(playlist: Playlist) -> Result<usize, HandlerError> {
    let ret_size: usize;
    let database_collection = InitCollectionDatabase {
        name: playlist.title,
        url: playlist.url,
        deezer_id: playlist.id.to_string(),
    };
    println!("Initializing collection");
    match domain::database::init_collection(database_collection) {
        Ok(size) => {
            ret_size = size;
        }
        Err(e) => {
            eprintln!("Error while initializing a collection in the database");
            return Err(HandlerError::HandlerDatabaseError(e));
        }
    };
    domain::database::add_tracks(playlist.tracks);
    return Ok(ret_size);
}

fn add_tracks_to_collection(playlist: Playlist) {
    match domain::database::get_collection_id_by_deezer_id(format!("{}", playlist.id)) {
        Ok(col_id) => playlist.tracks.into_iter().for_each(|track| {
            match domain::database::get_track_id_by_deezer_id(track.deezer_id.clone()) {
                Ok(track_id) => match domain::database::add_track_to_collection(col_id, track_id) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!(
                            "error adding track {track_id} to collection {col_id} : {:?}",
                            e
                        );
                    }
                },
                Err(e) => {
                    eprintln!(
                        "error getting track id from deezer id {} : {:?}",
                        track.deezer_id, e
                    );
                }
            }
        }),
        Err(e) => {
            eprintln!(
                "Error getting collection id from deezer_id {} : {:?}",
                playlist.id, e
            );
        }
    }
}

pub async fn get_playlist(playlist_id: u64) -> Result<Playlist, HandlerError> {
    println!("getting playlist {}", playlist_id);
    match domain::deezer::get_playlist(playlist_id).await {
        Ok(playlist) => Ok(convert_playlist(playlist)),
        Err(_) => Err(HandlerError::HandlerDeezerError()),
    }
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

pub fn get_playlist_id_from_url(url: String) -> u64 {
    let id_str: Vec<&str> = url.split('/').collect();
    return convert_string_to_u64(id_str.last().unwrap());
}

pub fn convert_string_to_u64(id: &&str) -> u64 {
    match id.parse::<u64>() {
        Ok(id) => return id,
        Err(e) => {
            println!("Error while converting id {} to u64", id);
            return 0;
        }
    }
}

pub fn get_track_id_from_url(url: String) -> u64 {
    let id_str: Vec<&str> = url.split('/').collect();
    match id_str.last().unwrap().parse::<u64>() {
        Ok(id) => return id,
        Err(e) => {
            println!("Error while parsing url {}", url);
            return 0;
        }
    }
}
