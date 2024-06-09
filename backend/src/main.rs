mod api;
mod domain;
mod handlers;

#[tokio::main]
async fn main() {
    let routes = api::routes::build_routes();
    println!("Server started at http://localhost:8000");
    warp::serve(routes).run(([0, 0, 0, 0], 8000)).await;
}
