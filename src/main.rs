use warp::{
    body::BodyDeserializeError, cors::CorsForbidden, http::Method, http::StatusCode,
    reject::Reject, Filter, Rejection, Reply,
};

async fn get_files() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status("Here is your file", StatusCode::OK))
}

#[tokio::main]
async fn main() {

    
    // let cors = warp::cors()
    //     .allow_any_origin()
    //     .allow_header("content-type")
    //     .allow_methods(&[Method::PUT, Method::POST, Method::GET, Method::DELETE]);

    let get_files = warp::get()
        .and(warp::path("image"))
        .and(warp::path("jpeg"))
        .and(warp::path("regular"))

        // .and(warp::path::end())
        .and(warp::fs::file("test.jpeg"));
        // .and(warp::query())
        // .and_then(get_files);
    let get_large_file = warp::get()
        .and(warp::path("image"))
        .and(warp::path("jpeg"))
        .and(warp::path("large"))

        // .and(warp::path::end())
        .and(warp::fs::file("large_jpeg.jpeg"));
    // let edit_question = warp::put()
    //     .and(warp::path("questions"))
    //     .and(warp::path::param::<String>())
    //     .and(warp::path::end())
    //     .and(store_filter.clone())
    //     .and(warp::body::json())
    //     .and_then(edit_question);


    let routes = get_files.or(get_large_file);
        // .with(cors)
        // .recover(return_error);

    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
}
