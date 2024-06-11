use std::env;

mod api;
mod common;
mod domain;
mod handlers;

#[tokio::main]
async fn main() {
    for arg in env::args().into_iter() {
        println!("arg : {}", arg);
    }
    let routes = api::routes::build_routes();
    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
