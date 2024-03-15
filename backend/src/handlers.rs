use std::convert::Infallible;
use warp::reply::Reply;

use crate::routes_models;

pub async fn init_collection(options : routes_models::InitCollectionInput) -> Result<impl Reply, Infallible> {
    println!("it works ! {:?}", options);
    return Ok(warp::reply());
}

// pub async fn list_collections() -> Result<impl Reply, Infallible> {

// }
// A route to handle GET requests for a specific post
// fn get_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path!("posts" / u64)
//         .and(warp::get())
//         .and_then(handlers::get_post)
// }