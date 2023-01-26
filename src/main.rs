use std::env;
use warp::path;
use warp::{ http::StatusCode, http::Uri, http::Method, Filter, reply::Reply};
use warp::Rejection;
use warp::reply::html;
use askama::Template; // bring trait in scope
use warp::reply::Response;
type WebResult<T> = std::result::Result<T, Rejection>;
#[derive(Template, Clone)] // this wiuse warp::reply::Response;ll generate the code...

#[template(path = "index.html")] // using the template in this path, relative                          // to the `templates` dir in the crate root
struct HelloTemplate<'a> { // the name of the struct can be anything
    site_url: &'a str, // the field name should match the variable name
                   // in your template
}
impl warp::Reply for HelloTemplate<'_> {
    fn into_response(self) -> warp::reply::Response {
        Response::new(format!("message: {}", self.site_url).into())
    }
}
async fn health_check() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status("Service is running", StatusCode::OK))
}

pub async fn welcome_handler() -> WebResult<impl Reply> {
    let path = env::var("LOCAL_TEST_URL");
    let template = HelloTemplate {
        site_url: &path.unwrap()
    };
    let res = template
        .render()
        .unwrap();
    Ok(html(res))
}

#[tokio::main]
async fn main() {
    let path = env::var("LOCAL_TEST_URL");
    let body = HelloTemplate { site_url: &path.clone().unwrap()}; // instantiate your struct
    body.render().unwrap();
    // let body = body.render().unwrap();
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET]);
    
    println!("the $PATH variable at the time of compiling was: {:?}", path.unwrap());

    let index = warp::path("home").and(warp::path::end()).and_then(welcome_handler);
    
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
