use warp::{filters::cors::Builder, Filter, Rejection, Reply};

use crate::handlers::{
    collection_dependencies::add_collection_dependency,
    collection_management::{
        get_collection_with_tracks, init_collections, list_collections, refresh_collection_handler,
    },
    handlers_models::InitCollection,
};

use super::api_models::{AddCollectionToParent, InitCollectionInput};

fn get_cors_config() -> Builder {
    return warp::cors()
        .allow_any_origin()
        .allow_headers(vec![
            "Access-Control-Allow-Origin",
            "Origin",
            "Accept",
            "X-Requested-With",
            "Content-Type",
        ])
        .allow_methods(vec!["GET", "POST"]);
}

pub fn build_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    init_collection()
        .or(get_collection_list())
        .or(get_collection_by_deezer_id())
        .or(add_collection_to_parent())
        .or(refresh_collection())
}

// `POST /collection/init`
pub fn init_collection() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection" / "init")
        .and(warp::post())
        .and(warp::body::json()) //JSON body
        .and(warp::body::content_length_limit(1024 * 16)) // Avoids huge payloads
        .and_then(call_init_collection)
        .with(&get_cors_config())
}

async fn call_init_collection(
    init_collection_input: InitCollectionInput,
) -> Result<impl Reply, Rejection> {
    let init_collections_data = InitCollection {
        name: init_collection_input.name,
        from_playlist: init_collection_input.from_playlist,
    };
    match init_collections(init_collections_data).await {
        Ok(_) => {
            let reply = warp::reply();
            Ok(warp::reply::with_header(
                reply,
                "Access-Control-Allow-Origin",
                "*",
            ))
        }
        Err(_) => Err(warp::reject()),
    }
}

// `GET /collection/list`
pub fn get_collection_list() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection" / "list")
        .and(warp::get()) // Avoids huge payloads
        .and_then(call_get_collection_list)
        .with(&get_cors_config())
}

async fn call_get_collection_list() -> Result<impl Reply, Rejection> {
    let collection_list = list_collections();
    match collection_list {
        Ok(collection_list) => {
            let reply = warp::reply::json(&collection_list).into_response();
            Ok(reply)
        }
        Err(_) => {
            eprintln!("Error while fetching the collection list");
            Err(warp::reject())
        }
    }
}

// GET /collection/<collection_deezer_id>
pub fn get_collection_by_deezer_id() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone
{
    warp::path!("collection" / String)
        .and(warp::get())
        .and_then(call_get_collection_by_deezer_id)
        .with(&get_cors_config())
}

async fn call_get_collection_by_deezer_id(deezer_id: String) -> Result<impl Reply, Rejection> {
    println!("getting collection by deezer id {}", deezer_id.clone());
    match get_collection_with_tracks(deezer_id.clone()).await {
        Ok(collection) => Ok(warp::reply::json(&collection).into_response()),
        Err(e) => {
            eprintln!(
                "Error while getting the collection by deezer id {} : {:?}",
                deezer_id, e
            );
            Err(warp::reject())
        }
    }
}

// POST /collection-management/add-collection
pub fn add_collection_to_parent() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection-management" / "add-collection")
        .and(warp::post())
        .and(warp::body::json()) //JSON body
        .and(warp::body::content_length_limit(1024 * 16)) // Avoids huge payloads
        .and_then(call_add_collection_to_parent)
        .with(&get_cors_config())
}

async fn call_add_collection_to_parent(
    add_collection_to_parent_input: AddCollectionToParent,
) -> Result<impl Reply, Rejection> {
    match add_collection_dependency(
        add_collection_to_parent_input.parent_collection_id,
        add_collection_to_parent_input.child_collection_id,
    )
    .await
    {
        Ok(_) => {
            let reply = warp::reply();
            Ok(warp::reply::with_header(
                reply,
                "Access-Control-Allow-Origin",
                "*",
            ))
        }
        Err(_) => Err(warp::reject()),
    }
}

// PUT /collection-management/refresh-collection/<collection-id>
pub fn refresh_collection() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection-management" / "refresh-collection" / String)
        .and(warp::put())
        .and_then(call_refresh_collection)
        .with(&get_cors_config())
}

async fn call_refresh_collection(collection_id: String) -> Result<impl Reply, Rejection> {
    match refresh_collection_handler(collection_id).await {
        Ok(_) => {
            let reply = warp::reply();
            Ok(warp::reply::with_header(
                reply,
                "Access-Control-Allow-Origin",
                "*",
            ))
        }
        Err(_) => Err(warp::reject()),
    }
}

// pub fn get_deezer_playlist() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
//     warp::path!("playlist")
//         .and(warp::get()) // Avoids huge payloads
//         .and_then(call_get_playlist)
//         .with(&get_cors_config())
// }

// async fn call_get_playlist() -> Result<impl Reply, Rejection> {
//     let playlist = collection_management::get_playlist(11043374682).await;
//     match playlist {
//         Ok(playlist) => {
//             let reply = warp::reply::json(&playlist).into_response();
//             Ok(reply)
//         }
//         Err(_) => {
//             eprintln!("Error while fetching the playlist");
//             Err(warp::reject())
//         }
//     }
// }
