use warp::{http::StatusCode, Filter};

#[tokio::main]
async fn main() {
    let health_route = warp::path!("health")
        .map(|| StatusCode::OK);
    let routes = health_route
        .with(warp::cors().allow_any_origin());
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}