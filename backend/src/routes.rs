use warp::{Filter, Rejection, Reply};
use crate::handlers;

pub fn build_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    init_collection()
}

// `PUT /init/collection?collection_name=<collection_name>&playlist_name=<playlist_name>`
pub fn init_collection() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("init/collection" / u64)
        .and(warp::post())
        .and_then(handlers::init_collection)
}

// `GET /collection/list`
pub fn list_collections() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection/list" / u64)
        .and(warp::post())
        .and_then(handlers::list_collections)
}