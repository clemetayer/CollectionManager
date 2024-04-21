use deezer::models::{DeezerArray, PlaylistTrack};

use super::handlers_models::{self, CollectionListElement, Playlist, Track};
use crate::domain::deezer::create_playlist;
use crate::domain::{self, domain_models::InitCollectionDatabase};
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

async fn create_new_playlist(name: String) -> Result<usize, HandlerError> {
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

async fn create_collection_from_playlist(playlist_id: u64) -> Result<usize, HandlerError> {
    let ret_size: usize;
    println!("Creating collection from deezer playlist");
    match get_playlist(playlist_id).await {
        Ok(playlist) => {
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
        }
        Err(e) => {
            eprintln!("Error getting playlist {} : {:?}", playlist_id.clone(), e);
            return Err(HandlerError::HandlerDeezerError());
        }
    }
    return Ok(ret_size);
}

fn get_playlist_id_from_url(url: String) -> u64 {
    let id_str: Vec<&str> = url.split('/').collect();
    match id_str.last().unwrap().parse::<u64>() {
        Ok(id) => return id,
        Err(e) => {
            println!("Error while parsing url {}", url);
            return 0;
        }
    }
}

fn get_track_id_from_url(url: String) -> u64 {
    let id_str: Vec<&str> = url.split('/').collect();
    match id_str.last().unwrap().parse::<u64>() {
        Ok(id) => return id,
        Err(e) => {
            println!("Error while parsing url {}", url);
            return 0;
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
