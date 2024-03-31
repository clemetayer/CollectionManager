use warp::{Filter, Rejection, Reply};

use crate::handlers::{
    collection_management::{init_collections, list_collections},
    handlers_models::InitCollection,
};

use super::api_models::InitCollectionInput;

pub fn build_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    init_collection().or(get_collection_list())
}

// `POST /collection/init`
pub fn init_collection() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection" / "init")
        .and(warp::post())
        .and(warp::body::json()) //JSON body
        .and(warp::body::content_length_limit(1024 * 16)) // Avoids huge payloads
        .and_then(call_init_collection)
}

async fn call_init_collection(
    init_collection_input: InitCollectionInput,
) -> Result<impl Reply, Rejection> {
    let init_collections_data = InitCollection {
        name: init_collection_input.name,
        from_playlist: init_collection_input.from_playlist,
    };
    match init_collections(init_collections_data) {
        Ok(_) => Ok(warp::reply()),
        Err(_) => Err(warp::reject()),
    }
}

// `GET /collection/list`
pub fn get_collection_list() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection" / "list")
        .and(warp::get()) // Avoids huge payloads
        .and_then(call_get_collection_list)
}

async fn call_get_collection_list() -> Result<impl Reply, Rejection> {
    let collection_list = list_collections();
    match collection_list {
        Ok(collection_list) => Ok(warp::reply::json(&collection_list).into_response()),
        Err(_) => {
            eprintln!("Error while fetching the collection list");
            Err(warp::reject())
        }
    }
}
