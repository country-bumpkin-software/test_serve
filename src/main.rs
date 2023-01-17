use warp::{ http::StatusCode, Filter};

async fn health_check() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status("Service is running", StatusCode::OK))
}

#[tokio::main]
async fn main() {
    // let cors = warp::cors()
    //     .allow_any_origin()
    //     .allow_header("content-type")
    //     .allow_methods(&[Method::PUT, Method::POST, Method::GET, Method::DELETE]);
    
    let assets = warp::path("images")
        .and(warp::fs::dir("images"));
    
    let health_check  = warp::get()
        .and(warp::path("health")).and(warp::path::end()).and_then(health_check);

    let routes = health_check.or(assets);
        // .with(cors)
        // .recover(return_error);

    warp::serve(routes).run(([0, 0, 0, 0], 8088)).await;
}
