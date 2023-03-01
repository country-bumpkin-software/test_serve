use std::env;
use warp::http::Uri;
use warp::{ http::StatusCode, http::Method, Filter, reply::Reply};
use warp::Rejection;
use warp::reply::html;
use askama::Template;
use warp::reply::Response;
use serde::Deserialize;

type WebResult<T> = std::result::Result<T, Rejection>;
#[derive(Deserialize, Debug, Clone)]
struct EnvConfiguration {
    base_url: String,
}

#[derive(Template, Clone)] // this will use warp::reply::Response and will generate the code...

#[template(path = "index.html")] // using the template in this path, relative to the `templates` dir in the crate root
struct BaseUrl<'a> {
    site_url: &'a str, // the field name should match the env variable name
                   
}
impl warp::Reply for BaseUrl<'_> {
    fn into_response(self) -> warp::reply::Response {
        Response::new(format!("message: {}", self.site_url).into())
    }
}
async fn health_check() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status("Service is running", StatusCode::OK))
}

pub async fn home_handler() -> WebResult<impl Reply> {
    let path = env::var("BASE_URL");
    let template = BaseUrl {
        site_url: &path.unwrap()
    };
    let res = template
        .render()
        .unwrap();
    Ok(html(res))
}

#[tokio::main]
async fn main() {
    let path = env::var("BASE_URL");

    let body = BaseUrl { site_url: &path.clone().unwrap()}; // instantiate your struct
    body.render().unwrap();
    // let body = body.render().unwrap();
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET]);
    
    // println!("the $PATH variable at the time of compiling was: {:?}", path.unwrap());

    let index = warp::path("home").and(warp::path::end()).and_then(home_handler);
    
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
            } else if reply.path().ends_with("file_example_AVI_1280_1_5MG.avi") {
                println!("1{:?}", reply);
                warp::reply::with_header(reply, "Content-Type", "video/avi").into_response()
            } else if reply.path().ends_with("sample_960x540.mp4") {
                println!("1{:?}", reply);
                warp::reply::with_header(reply, "Content-Type", "video/mpeg").into_response()
            }else {
                println!("else {:?}", reply);
                reply.into_response()
            }
        });
        // file_example_XMSVIDEO_1280_1_5MG
    
    let health_check  = warp::get()
        .and(warp::path("health")).and(warp::path::end()).and_then(health_check);
    
    
    let mut redirect_path = envy::from_env::<EnvConfiguration>()
    .expect("Please provide BASE_URL env vars");
    redirect_path = EnvConfiguration{base_url: redirect_path.base_url.to_owned() + "images/redirect.jpeg"};
    
    println!("base url is: {:?}", redirect_path.base_url);
    let redirect_route = warp::path("will_redirect").map(move|| {
        let uri = redirect_path.base_url.parse::<Uri>().unwrap();
        print!("uri: {:?}", uri);
        warp::redirect(uri)
    });

   

    let routes = health_check.or(assets).or(index).or(redirect_route)
        .with(cors);
        // .recover(return_error);

    warp::serve(routes).run(([0, 0, 0, 0], 8088)).await;
}
