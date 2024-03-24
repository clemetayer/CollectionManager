use std::convert::Infallible;

use warp::{Filter, Rejection, Reply};

use crate::handlers::{collection_management::init_collections, handlers_models::InitCollection};

use super::api_models::InitCollectionInput;

pub fn build_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    init_collection()
}

// `PUT /init/collection?collection_name=<collection_name>&playlist_name=<playlist_name>`
pub fn init_collection() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("init" / "collection")
        .and(warp::post())
        .and(warp::body::json()) //JSON body
        .and(warp::body::content_length_limit(1024 * 16)) // Avoids huge payloads
        .and_then(call_init_collection)
}

async fn call_init_collection(init_collection_input : InitCollectionInput) -> Result<impl Reply, Infallible>   {
    let init_collections_data = InitCollection {
        name: init_collection_input.name,
        from_playlist: init_collection_input.from_playlist
    };
    return init_collections(init_collections_data).await;
}