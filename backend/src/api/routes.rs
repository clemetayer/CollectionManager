use super::api_models::{AddCollectionToParent, InitCollectionInput, RemoveCollectionToParent};
use crate::domain::{
    collection_dependencies::{add_collection_dependency, remove_collection_dependency},
    collection_management::{
        clear_data as clear_data_domain, get_collection,
        get_collection_tracks_excluding_children as get_collection_tracks_excluding_children_domain,
        get_direct_children_collections as get_direct_children_collections_domain,
        init_collections, list_collections, refresh_collection as refresh_collection_domain,
        remove_collection as remove_collection_domain, update_all_collections,
    },
    domain_models::InitCollection,
    errors::DomainError,
};
use log::info;
use warp::{filters::cors::Builder, reply::Response, Filter, Rejection, Reply};

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

/// POST /collection/init
///
/// initializes a collection
///     - in deezer and in the database if from_playlist is empty
///     - in the database only, from an existing deezer playlist if from_playlist is set
///
/// inputs : Json with body
/// {
///     "name":String, // name of the collection. will be ignored if from_playlist is set.
///     "from_playlist":Option<String> // url of the deezer playlist
/// }
///
/// outputs : empty
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
) -> Result<Response, Rejection> {
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
            Ok(warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*").into_response())
        }
        Err(e) => Ok(handle_domain_errors(e)),
    }
}

/// GET /collection/list
///
/// Returns a list of all the collections saved in the database
///
/// inputs : empty
///
/// outputs : list of collections
pub fn get_collection_list() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection" / "list")
        .and(warp::get()) // Avoids huge payloads
        .and_then(call_get_collection_list)
        .with(&get_cors_config())
}

async fn call_get_collection_list() -> Result<Response, Rejection> {
    info!("getting collection list");
    let collection_list = list_collections();
    match collection_list {
        Ok(collection_list) => {
            let reply = warp::reply::json(&collection_list).into_response();
            Ok(reply)
        }
        Err(e) => Ok(handle_domain_errors(e)),
    }
}

/// GET /collection/tracks/<collection-id>
///
/// Returns the track from a collection, but without the tracks from the children collections
///
/// inputs : deezer playlist id as a String
///
/// outputs : list of tracks
pub fn get_collection_tracks_excluding_children(
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection" / "tracks" / String)
        .and(warp::get())
        .and_then(call_get_collection_tracks_excluding_children)
        .with(&get_cors_config())
}

async fn call_get_collection_tracks_excluding_children(id: String) -> Result<Response, Rejection> {
    info!("getting tracks from collection {}", id);
    match get_collection_tracks_excluding_children_domain(id.as_str()).await {
        Ok(tracks) => Ok(warp::reply::json(&tracks).into_response()),
        Err(e) => Ok(handle_domain_errors(e)),
    }
}

/// GET /collection/<collection_id>
///
/// Returns the data of a collection by its deezer playlist id
///
/// inputs : deezer playlist id as a String
///
/// outputs : collection data
pub fn get_collection_by_id() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection" / String)
        .and(warp::get())
        .and_then(call_get_collection_by_id)
        .with(&get_cors_config())
}

async fn call_get_collection_by_id(id: String) -> Result<Response, Rejection> {
    info!("getting collection by id {}", id);
    match get_collection(id.as_str()).await {
        Ok(collection) => Ok(warp::reply::json(&collection).into_response()),
        Err(e) => Ok(handle_domain_errors(e)),
    }
}

/// GET /collection-management/children/<collection-id>
///
/// Returns the children of a collection
///
/// inputs : deezer playlist id as a String
///
/// outputs : list of the children collections
pub fn get_direct_children_collections(
) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection-management" / "children" / String)
        .and(warp::get())
        .and_then(call_get_direct_children_collections)
        .with(&get_cors_config())
}

async fn call_get_direct_children_collections(id: String) -> Result<Response, Rejection> {
    info!("getting children collections of id {}", id);
    match get_direct_children_collections_domain(id.as_str()) {
        Ok(collections) => Ok(warp::reply::json(&collections).into_response()),
        Err(e) => Ok(handle_domain_errors(e)),
    }
}

/// POST /collection-management/add-collection
///
/// Adds a collection in another collection (child in parent)
///
/// inputs : Json with body
/// {
///     "parent_collection_id": String, // parent deezer playlist id
///     "child_collection_id": String // child deezer playlist id
/// }
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
) -> Result<Response, Rejection> {
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
            Ok(warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*").into_response())
        }
        Err(e) => Ok(handle_domain_errors(e)),
    }
}

/// PUT /collection-management/refresh-collection/<collection-id>
///
/// Refreshes a collection, i.e, adds the tracks from a child collection into the parent collection in deezer
///
/// inputs : deezer id as a String
///
/// outputs : nothing
pub fn refresh_collection() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection-management" / "refresh-collection" / String)
        .and(warp::put())
        .and_then(call_refresh_collection)
        .with(&get_cors_config())
}

async fn call_refresh_collection(collection_id: String) -> Result<Response, Rejection> {
    info!("refreshing collection {}", collection_id);
    match refresh_collection_domain(collection_id.as_str()).await {
        Ok(_) => {
            let reply = warp::reply();
            Ok(warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*").into_response())
        }
        Err(e) => Ok(handle_domain_errors(e)),
    }
}

/// PUT /collection-management/refresh-all-collections
///
/// Refreshes all the collections, i.e, adds the tracks from a child collection into the parent collection in deezer
///
/// inputs : empty
///
/// outputs : empty
pub fn refresh_all_collections() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection-management" / "refresh-all-collections")
        .and(warp::put())
        .and_then(call_refresh_all_collections)
        .with(&get_cors_config())
}

async fn call_refresh_all_collections() -> Result<Response, Rejection> {
    info!("refreshing all collections");
    match update_all_collections().await {
        Ok(_) => {
            let reply = warp::reply();
            Ok(warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*").into_response())
        }
        Err(e) => Ok(handle_domain_errors(e)),
    }
}

/// DELETE /collection-management/remove-collection
///
/// Removes a collection from another collection (i.e removes a child collection from its parent)
/// Warning : this will only affect the database, you will have to remove the associated tracks from deezer manually
///
/// inputs : Json with body
/// {
///     "parent_collection_id": String, // parent deezer playlist id
///     "child_collection_id": String // child deezer playlist id
/// }
///
/// outputs : empty
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
) -> Result<Response, Rejection> {
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
            Ok(warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*").into_response())
        }
        Err(e) => Ok(handle_domain_errors(e)),
    }
}

/// DELETE /collection/<collection-id>
///
/// Removes a collection
/// Warning : this will only affect the database, you will have to remove the associated tracks from deezer manually
///
/// inputs : deezer playlist id as a String
///
/// outputs : empty
pub fn remove_collection() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("collection" / String)
        .and(warp::delete())
        .and_then(call_remove_collection)
        .with(&get_cors_config())
}

async fn call_remove_collection(collection_id: String) -> Result<Response, Rejection> {
    info!("removing collection {}", collection_id);
    match remove_collection_domain(collection_id.as_str()) {
        Ok(_) => {
            let reply = warp::reply();
            Ok(warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*").into_response())
        }
        Err(e) => Ok(handle_domain_errors(e)),
    }
}

/// DELETE /clear-data
///
/// Resets the database
/// Should only be used for integration tests, or if you want a full reset of your data
/// Warning : this will only affect the database, you will have to remove the associated tracks from deezer manually
///
/// inputs : empty
///
/// outputs : empty
pub fn clear_data() -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("clear-data")
        .and(warp::delete())
        .and_then(call_clear_data)
        .with(&get_cors_config())
}

async fn call_clear_data() -> Result<Response, Rejection> {
    info!("clearing data");
    match clear_data_domain() {
        Ok(_) => {
            let reply = warp::reply();
            Ok(warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*").into_response())
        }
        Err(e) => Ok(handle_domain_errors(e)),
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

fn handle_domain_errors(e: DomainError) -> Response {
    match e {
        crate::domain::errors::DomainError::DomainParamError() => {
            return warp::reply::with_status("BAD_REQUEST", warp::http::StatusCode::BAD_REQUEST)
                .into_response();
        }
        crate::domain::errors::DomainError::DomainDataError()
        | crate::domain::errors::DomainError::DomainMusicServiceError() => {
            return warp::reply::with_status(
                "INTERNAL_SERVER_ERROR",
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )
            .into_response();
        }
    }
}
