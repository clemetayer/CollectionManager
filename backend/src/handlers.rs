pub async fn init_collection(options : Options) -> Result<impl Reply, Infallible> {
    
}

pub async fn list_collections() -> Result<impl Reply, Infallible> {

}
// A route to handle GET requests for a specific post
// fn get_post() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
//     warp::path!("posts" / u64)
//         .and(warp::get())
//         .and_then(handlers::get_post)
// }