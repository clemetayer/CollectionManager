use super::domain_models::InitCollection;

const API_PATH_PLAYLIST: &str = "https://www.deezer.com/fr/playlist/";

pub fn check_init_collections(params: &InitCollection) -> String {
    let mut res: String = "".to_owned();
    match &params.from_playlist {
        Some(playlist_url) => {
            if !playlist_url.starts_with(API_PATH_PLAYLIST) {
                res.push_str(
                    "- from_playlist does not start with https://www.deezer.com/fr/playlist/\n",
                );
            }
        }
        None => {}
    }
    return res;
}

pub fn check_id_valid(id: String) -> String {
    match id.parse::<u64>() {
        Ok(_) => return "".to_owned(),
        Err(_) => {
            return "- id not valid (should be a 64 bits unsigned integer)".to_owned();
        }
    }
}
