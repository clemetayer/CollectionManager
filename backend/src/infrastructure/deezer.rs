use std::time::Duration;

use deezer::models::Playlist;
use log::warn;
use ratelimit::Ratelimiter;
use reqwest::Error;

use crate::common::common::get_env_variable;

use super::deezer_models::CreatedPlaylist;

const PATH_PLAYLIST: &str = "playlist";
const PATH_TRACKS: &str = "tracks";
const PATH_USER: &str = "user";
const PATH_PLAYLISTS: &str = "playlists";

static mut RATELIMITER: Option<Ratelimiter> = None;

fn get_deezer_api_path() -> String {
    return get_env_variable("DEEZER_API_URL");
}

fn get_user_id() -> String {
    return get_env_variable("DEEZER_USER_ID");
}

fn get_token() -> String {
    return get_env_variable("DEEZER_API_TOKEN");
}

fn string_to_u64(to_convert: &str) -> u64 {
    return to_convert.parse::<u64>().unwrap();
}

fn init_rate_limiter() {
    let rate_limit_amount = string_to_u64(get_env_variable("RATE_LIMIT").as_str());
    let rate_limit_timeout = string_to_u64(get_env_variable("RATE_LIMIT_TIMEOUT").as_str());
    unsafe {
        RATELIMITER = Some(
            Ratelimiter::builder(rate_limit_amount, Duration::from_secs(rate_limit_timeout))
                .max_tokens(rate_limit_amount)
                .build()
                .unwrap(),
        );
    };
}

fn limit_rate_if_needed() {
    unsafe {
        match &RATELIMITER {
            Some(ratelimit) => {
                if let Err(sleep) = ratelimit.try_wait() {
                    warn!("Too many requests to the deezer API, limiting the rates.");
                    std::thread::sleep(sleep);
                    return;
                }
            }
            None => init_rate_limiter(),
        }
    }
}

pub async fn create_playlist(name: &str) -> Result<u64, Error> {
    let mut url: String = format!(
        "{}/{}/{}/{}?{}",
        get_deezer_api_path(),
        PATH_USER,
        get_user_id(),
        PATH_PLAYLISTS,
        format!("title={}", name)
    );
    let token = get_token();
    if token != "" {
        url = format!("{}&access_token={}", url, token);
    }
    limit_rate_if_needed();
    let client = reqwest::Client::new();
    let response: reqwest::Response = client.post(url).header("content-length", 0).send().await?;
    match response.json::<CreatedPlaylist>().await {
        Ok(playlist) => Ok(playlist.id),
        Err(e) => {
            eprintln!(
                "Error querying {}/{}/{}/{}?{}&{} : {}",
                get_deezer_api_path(),
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

pub async fn get_playlist(deezer_playlist_id: &u64) -> Result<Playlist, Error> {
    let mut url: String = format!(
        "{}/{}/{}",
        get_deezer_api_path(),
        PATH_PLAYLIST,
        deezer_playlist_id,
    );
    let token = get_token();
    if token != "" {
        url = format!("{}?access_token={}", url, token);
    }
    limit_rate_if_needed();
    let client = reqwest::Client::new();
    let response = client.get(url).send().await?;
    match response.json::<deezer::models::Playlist>().await {
        Ok(playlist) => Ok(playlist),
        Err(e) => {
            eprintln!(
                "Error querying {}/{}/{} : {}",
                get_deezer_api_path(),
                PATH_PLAYLIST,
                deezer_playlist_id,
                e
            );
            Err(e)
        }
    }
}

pub async fn add_tracks_to_playlist(
    playlist_id: &str,
    track_ids: Vec<String>,
) -> Result<bool, Error> {
    if track_ids.len() > 0 {
        let mut track_ids_query_params: String = "".to_owned();
        for track_id in track_ids.into_iter() {
            track_ids_query_params.push_str(&format!("{},", track_id));
        }
        track_ids_query_params.pop(); // removes the last comma
        let mut url: String = format!(
            "{}/{}/{}/{}?{}",
            get_deezer_api_path(),
            PATH_PLAYLIST,
            playlist_id,
            PATH_TRACKS,
            format!("songs={}", track_ids_query_params)
        );
        let token = get_token();
        if token != "" {
            url = format!("{}&access_token={}", url, token);
        }
        limit_rate_if_needed();
        let client = reqwest::Client::new();
        let response = client.post(url).header("content-length", 0).send().await?;
        match response.json::<bool>().await {
            Ok(value) => return Ok(value),
            Err(e) => {
                eprintln!(
                    "Error querying {}/{}/{}/{}?{} : {}",
                    get_deezer_api_path(),
                    PATH_PLAYLIST,
                    playlist_id,
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
