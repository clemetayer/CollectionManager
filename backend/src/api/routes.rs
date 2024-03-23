use warp::{Filter, Rejection, Reply};
use crate::handlers;

pub fn build_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    init_collection()
}

// `PUT /init/collection?collection_name=<collection_name>&playlist_name=<playlist_name>`
pub fn init_collection() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("init" / "collection")
        .and(warp::post())
        .and(warp::body::json()) //JSON body
        .and(warp::body::content_length_limit(1024 * 16)) // Avoids huge payloads
        .and_then(handlers::collection_management::init_collection)
}
