use warp::{ http::StatusCode, http::Uri, http::Method, Filter, reply::Reply};
async fn health_check() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status("Service is running", StatusCode::OK))
}

#[tokio::main]
async fn main() {
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET]);

    let mut body = r#"
    <html>
        <head>
            <title>Test Files</title>
        </head>
        <body>
            <h1>Test Images</h1>
            <h2>JPEG</h2>
            <a href="https://test-data-serve.onrender.com/images/valid.jpeg">small jpeg</a>
            <a href="https://test-data-serve.onrender.com/images/17mb.jpeg">large 17mb jpeg</a>
            <a href="https://test-data-serve.onrender.com/images/24mb.jpeg">large 24mb jpeg</a>
            <a href="https://test-data-serve.onrender.com/images/28mb.jpeg">larger 28mb jpeg</a>
            <a href="https://test-data-serve.onrender.com/images/40mb.jpeg">larger 40mb jpeg</a>
            <h2>PNG</h2>
            <a href="https://test-data-serve.onrender.com/images/PNG_Test.png">test png</a>
            <h2>SVG</h2>
            <a href="https://test-data-serve.onrender.com/images/SVG_Test.svg">test svg</a>
            <h2>HEIC</h2>
            <a href="https://test-data-serve.onrender.com/images/HEIC_GOOD.heic">good test heic</a>
            <h2>Invalid application/octet-stream</h2>
            <a href="https://test-data-serve.onrender.com/images/HEIC_Test.heic">test heic - invalid content-type</a>
            <h2>Redirect (to valid jpeg image)</h2>
            <a href="https://test-data-serve.onrender.com/will_redirect">will redirect</a>
            <h2>Cors (jpeg image)</h2>
            <a href="https://test-data-serve.onrender.com/images/cors.jpeg">cors denied</a>
            <a href="https://test-data-serve.onrender.com/images/cors.jpeg">cors allowed</a>
            <h2>Valid 240 Mega Pixel Image</h2>
            <a href="https://test-data-serve.onrender.com/images/valid_240mp.jpeg">Valid 240 Mega Pixel Image</a>
            <h2>Invalid 256 Mega Pixel Image</h2>
            <a href="https://test-data-serve.onrender.com/images/invalid_256mp.jpeg">Invalid 256 Mega Pixel Image</a>
            </body>
    </html>
    "#;

    let index = warp::path("home").and(warp::path::end()).map(move|| {
        warp::reply::html(body)
    });
    
    let assets = warp::path("images")
        .and(warp::fs::dir("images")).map( |reply: warp::filters::fs::File| {
            print!("{:?}", reply);
            if reply.path().ends_with("HEIC_GOOD.heic") {
                println!("1{:?}", reply);
                warp::reply::with_header(reply, "Content-Type", "image/heic").into_response()
            } else if reply.path().ends_with("cors.jpeg") {
                println!("1{:?}", reply);
                warp::reply::with_header(reply, "Access-Control-Allow-Origin", "https://developer.mozilla.org").into_response()
            } else if reply.path().ends_with("cors_allowed.jpeg") {
                println!("1{:?}", reply);
                warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*").into_response()
            } else {
                println!("else {:?}", reply);
                reply.into_response()
            }
        });
    
    let health_check  = warp::get()
        .and(warp::path("health")).and(warp::path::end()).and_then(health_check);

    let redirect_route = warp::path("will_redirect").map(|| {
        warp::redirect(Uri::from_static("https://test-data-serve.onrender.com/images/valid.jpeg"))
    });

    let routes = health_check.or(assets).or(index).or(redirect_route)
        .with(cors);
        // .recover(return_error);

    warp::serve(routes).run(([0, 0, 0, 0], 8088)).await;
}
