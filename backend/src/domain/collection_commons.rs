use super::converter::convert_playlist;
use super::domain_models::Playlist;
use super::errors::DomainError;
use crate::infrastructure;
use crate::infrastructure::database::get_collection_id_by_deezer_id as get_collection_id_by_deezer_id_database;
use crate::infrastructure::database_models::InitCollectionDatabase;
use crate::infrastructure::deezer::create_playlist;
use log::error;

pub async fn create_new_playlist(name: &str) -> Result<bool, DomainError> {
    match create_playlist(name).await {
        Ok(id) => {
            let database_collection = InitCollectionDatabase {
                name: name.to_string(),
                url: format!("https://www.deezer.com/fr/playlist/{}", id),
                deezer_id: format!("{}", id),
            };
            match infrastructure::database::init_collection(database_collection) {
                Ok(_) => {}
                Err(e) => {
                    return Err(log_database_error(&format!(
                        "Error while initializing a collection {} in the database : {:?}",
                        name, e
                    )));
                }
            };
        }
        Err(e) => {
            return Err(log_deezer_error(&format!(
                "Error creating playlist {} : {:?}",
                name, e
            )));
        }
    }
    return Ok(true);
}

pub async fn create_collection_from_playlist(playlist_id: &u64) -> Result<bool, DomainError> {
    match get_playlist(playlist_id).await {
        Ok(playlist) => {
            add_playlist_data_to_database(playlist)?;
        }
        Err(e) => {
            return Err(log_deezer_error(&format!(
                "Error getting playlist {} : {:?}",
                playlist_id, e
            )));
        }
    }
    return Ok(true);
}

pub async fn get_playlist(playlist_id: &u64) -> Result<Playlist, DomainError> {
    match infrastructure::deezer::get_playlist(playlist_id).await {
        Ok(playlist) => Ok(convert_playlist(playlist)),
        Err(e) => {
            return Err(log_deezer_error(&format!(
                "Error getting playlist {} : {:?}",
                playlist_id, e
            )))
        }
    }
}

pub fn get_playlist_id_from_url(url: String) -> u64 {
    let id_str: Vec<&str> = url.split('/').collect();
    return convert_string_to_u64(id_str.last().unwrap());
}

pub fn convert_string_to_u64(id: &str) -> u64 {
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

pub fn get_collection_id_by_deezer_id(deezer_id: &str) -> Result<i32, DomainError> {
    match get_collection_id_by_deezer_id_database(deezer_id) {
        Ok(id) => {
            return Ok(id);
        }
        Err(e) => {
            return Err(log_database_error(&format!(
                "Error while getting collection id {} : {:?}",
                deezer_id, e
            )));
        }
    };
}

pub fn log_parameters_error(failures: String) -> Result<bool, DomainError> {
    if !failures.is_empty() {
        error!("Handler - error in input parameters : {}", failures);
        return Err(DomainError::DomainParamError());
    }
    return Ok(true);
}

pub fn log_database_error(message: &str) -> DomainError {
    error!("Handler : {}", message);
    return DomainError::DomainDataError();
}

pub fn log_deezer_error(message: &str) -> DomainError {
    error!("Handler : {}", message);
    return DomainError::DomainMusicServiceError();
}

fn add_playlist_data_to_database(playlist: Playlist) -> Result<bool, DomainError> {
    let database_collection = InitCollectionDatabase {
        name: playlist.title,
        url: playlist.url.clone(),
        deezer_id: playlist.id.to_string(),
    };
    match infrastructure::database::init_collection(database_collection) {
        Ok(_) => {}
        Err(e) => {
            return Err(log_database_error(&format!(
                "Error while initializing the collection {} in the database : {:?}",
                playlist.url, e
            )));
        }
    };
    return Ok(true);
}
