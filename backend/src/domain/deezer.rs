use deezer::models::Playlist;
use dotenvy::dotenv;
use reqwest::Error;
use std::env;

use super::deezer_models::CreatedPlaylist;

const PATH_DEEZER_API: &str = "https://api.deezer.com";
const PATH_PLAYLIST: &str = "playlist";
const PATH_TRACKS: &str = "tracks";
const PATH_USER: &str = "user";
const PATH_PLAYLISTS: &str = "playlists";

fn get_user_id() -> String {
    dotenv().ok();
    return env::var("DEEZER_USER_ID").expect("DEEZER_USER_ID must be set");
}

fn get_token() -> String {
    dotenv().ok();
    return env::var("DEEZER_API_TOKEN").expect("DEEZER_API_TOKEN must be set");
}

pub async fn create_playlist(name: String) -> Result<u64, Error> {
    let url: String = format!(
        "{}/{}/{}/{}?{}&{}",
        PATH_DEEZER_API,
        PATH_USER,
        get_user_id(),
        PATH_PLAYLISTS,
        format!("title={}", name),
        format!("access_token={}", get_token())
    );
    let client = reqwest::Client::new();
    let response: reqwest::Response = client.post(url).header("content-length", 0).send().await?;
    match response.json::<CreatedPlaylist>().await {
        Ok(playlist) => Ok(playlist.id),
        Err(e) => {
            eprintln!(
                "Error querying {}/{}/{}/{}?{}&{} : {}",
                PATH_DEEZER_API,
                PATH_USER,
                get_user_id(),
                PATH_PLAYLISTS,
                format!("title={}", name),
                format!("access_token={}", get_token()),
                e
            );
            Err(e)
        }
    }
}

pub async fn get_playlist(deezer_playlist_id: u64) -> Result<Playlist, Error> {
    let url: String = format!(
        "{}/{}/{}?{}",
        PATH_DEEZER_API,
        PATH_PLAYLIST,
        deezer_playlist_id,
        format!("access_token={}", get_token())
    );
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    match response.json::<deezer::models::Playlist>().await {
        Ok(playlist) => Ok(playlist),
        Err(e) => {
            eprintln!(
                "Error querying {}/{}/{} : {}",
                PATH_DEEZER_API, PATH_PLAYLIST, deezer_playlist_id, e
            );
            Err(e)
        }
    }
}

pub async fn add_tracks_to_playlist(
    playlist_id: String,
    track_ids: Vec<String>,
) -> Result<bool, Error> {
    if track_ids.len() > 0 {
        let mut track_ids_query_params: String = "".to_owned();
        for track_id in track_ids.into_iter() {
            track_ids_query_params.push_str(&format!("{},", track_id));
        }
        let url: String = format!(
            "{}/{}/{}/{}?{}",
            PATH_DEEZER_API,
            PATH_PLAYLIST,
            playlist_id.clone(),
            PATH_TRACKS,
            format!(
                "access_token={}&songs={}",
                get_token(),
                track_ids_query_params
            )
        );
        let client = reqwest::Client::new();
        let response = client.post(url).header("content-length", 0).send().await?;
        match response.json::<bool>().await {
            Ok(value) => return Ok(value),
            Err(e) => {
                eprintln!(
                    "Error querying {}/{}/{}/{}?{} : {}",
                    PATH_DEEZER_API,
                    PATH_PLAYLIST,
                    playlist_id.clone(),
                    PATH_TRACKS,
                    format!(
                        "access_token={}&songs={}",
                        get_token(),
                        track_ids_query_params
                    ),
                    e
                );
                return Err(e);
            }
        }
    }
    return Ok(false);
}
