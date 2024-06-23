use super::api_models::{AddCollectionToParent, InitCollectionInput, RemoveCollectionToParent};
use crate::domain::{
    collection_dependencies::{add_collection_dependency, remove_collection_dependency},
    collection_management::{
        clear_data as clear_data_domain, get_collection,
        get_collection_tracks_excluding_children as get_collection_tracks_excluding_children_domain,
        get_direct_children_collections as get_direct_children_collections_domain,
        init_collections, list_collections, refresh_collection_handler, remove_collection_handler,
        update_all_collections,
    },
    domain_models::InitCollection,
};
use log::info;
use warp::{filters::cors::Builder, Filter, Rejection, Reply};

pub fn build_routes() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    init_collection()
        .or(get_collection_list())
        .or(get_collection_by_id())
        .or(add_collection_to_parent())
        .or(refresh_collection())
        .or(refresh_all_collections())
        .or(remove_collection_from_parent())
        .or(remove_collection())
        .or(get_collection_tracks_excluding_children())
        .or(get_direct_children_collections())
        .or(clear_data())
}

// POST /collection/init
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
    info!(
        "initializing collection {}",
        init_collection_input.clone().name
    );
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

// GET /collection/list
pub fn get_collection_list() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection" / "list")
        .and(warp::get()) // Avoids huge payloads
        .and_then(call_get_collection_list)
        .with(&get_cors_config())
}

async fn call_get_collection_list() -> Result<impl Reply, Rejection> {
    info!("getting collection list");
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

// GET /collection/tracks/<collection-id>
pub fn get_collection_tracks_excluding_children(
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection" / "tracks" / String)
        .and(warp::get())
        .and_then(call_get_collection_tracks_excluding_children)
        .with(&get_cors_config())
}

async fn call_get_collection_tracks_excluding_children(
    id: String,
) -> Result<impl Reply, Rejection> {
    info!("getting tracks from collection {}", id);
    match get_collection_tracks_excluding_children_domain(id.as_str()).await {
        Ok(tracks) => Ok(warp::reply::json(&tracks).into_response()),
        Err(_) => Err(warp::reject()),
    }
}

// GET /collection/<collection_id>
pub fn get_collection_by_id() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection" / String)
        .and(warp::get())
        .and_then(call_get_collection_by_id)
        .with(&get_cors_config())
}

async fn call_get_collection_by_id(id: String) -> Result<impl Reply, Rejection> {
    info!("getting collection by id {}", id);
    match get_collection(id.as_str()).await {
        Ok(collection) => Ok(warp::reply::json(&collection).into_response()),
        Err(_) => Err(warp::reject()),
    }
}

// GET /collection-management/children/<collection-id>
pub fn get_direct_children_collections(
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection-management" / "children" / String)
        .and(warp::get())
        .and_then(call_get_direct_children_collections)
        .with(&get_cors_config())
}

async fn call_get_direct_children_collections(id: String) -> Result<impl Reply, Rejection> {
    info!("getting children collections of id {}", id);
    match get_direct_children_collections_domain(id.as_str()) {
        Ok(collections) => Ok(warp::reply::json(&collections).into_response()),
        Err(_) => Err(warp::reject()),
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
    info!(
        "adding collection {} to {}",
        &add_collection_to_parent_input.child_collection_id,
        &add_collection_to_parent_input.parent_collection_id
    );
    match add_collection_dependency(
        add_collection_to_parent_input.parent_collection_id.as_str(),
        add_collection_to_parent_input.child_collection_id.as_str(),
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
    info!("refreshing collection {}", collection_id);
    match refresh_collection_handler(collection_id.as_str()).await {
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

// PUT /collection-management/refresh-all-collections
pub fn refresh_all_collections() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection-management" / "refresh-all-collections")
        .and(warp::put())
        .and_then(call_refresh_all_collections)
        .with(&get_cors_config())
}

async fn call_refresh_all_collections() -> Result<impl Reply, Rejection> {
    info!("refreshing all collections");
    match update_all_collections().await {
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

// DELETE /collection-management/remove-collection
pub fn remove_collection_from_parent(
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection-management" / "remove-collection")
        .and(warp::delete())
        .and(warp::body::json()) //JSON body
        .and(warp::body::content_length_limit(1024 * 16)) // Avoids huge payloads
        .and_then(call_remove_collection_to_parent)
        .with(&get_cors_config())
}

async fn call_remove_collection_to_parent(
    remove_collection_to_parent_input: RemoveCollectionToParent,
) -> Result<impl Reply, Rejection> {
    info!(
        "removing collection {} from collection {}",
        remove_collection_to_parent_input.child_collection_id,
        remove_collection_to_parent_input.parent_collection_id
    );
    match remove_collection_dependency(
        remove_collection_to_parent_input
            .parent_collection_id
            .as_str(),
        remove_collection_to_parent_input
            .child_collection_id
            .as_str(),
    ) {
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

// DELETE /collection/<collection-id>
pub fn remove_collection() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection" / String)
        .and(warp::delete())
        .and_then(call_remove_collection)
        .with(&get_cors_config())
}

async fn call_remove_collection(collection_id: String) -> Result<impl Reply, Rejection> {
    info!("removing collection {}", collection_id);
    match remove_collection_handler(collection_id.as_str()) {
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

// DELETE /clear-data -- Should only be used for integration tests, or if you want a full reset of your data
pub fn clear_data() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("clear-data")
        .and(warp::delete())
        .and_then(call_clear_data)
        .with(&get_cors_config())
}

async fn call_clear_data() -> Result<impl Reply, Rejection> {
    info!("clearing data");
    match clear_data_domain() {
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
        .allow_methods(vec!["GET", "POST", "PUT", "DELETE"]);
}
