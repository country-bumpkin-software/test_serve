use std::env;
use warp::http::Uri;
use warp::{ http::StatusCode, http::Method, Filter, reply::Reply};
use warp::{Rejection, reply};
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

async fn forbidden_route() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status("You entered the forbidden realm", StatusCode::FORBIDDEN))
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
    let audio = warp::path("audio").and(warp::fs::dir("audio")).map(|reply: warp::filters::fs::File|
        if reply.path().ends_with("sample-12s.mp3") {
            warp::reply::with_header(reply, "Content-Type", "audio/mp3").into_response()
        } else if reply.path().ends_with("sample-mp4.mp4") {
            warp::reply::with_header(reply, "Content-Type", "audio/mp4").into_response()
        } else if reply.path().ends_with("sample-m4a.m4a") {
            warp::reply::with_header(reply, "Content-Type", "audio/mp4").into_response()
        }  else if reply.path().ends_with("sample-m4r.m4r") {
            warp::reply::with_header(reply, "Content-Type", "audio/mp4").into_response()
        } else if reply.path().ends_with("sample-m4v.m4v") {
            warp::reply::with_header(reply, "Content-Type", "audio/mp4").into_response()
        } else if reply.path().ends_with("sample-oversized-120mb.mp4") {
            warp::reply::with_header(reply, "Content-Type", "audio/mp4").into_response()
        } else if reply.path().ends_with("sample-12s-mpeg.mpeg") {
            warp::reply::with_header(reply, "Content-Type", "audio/mpeg").into_response()
        } else if reply.path().ends_with("sample3.mp2") {
            warp::reply::with_header(reply, "Content-Type", "audio/mpeg").into_response()
        }else if reply.path().ends_with("32ch-44100-bwf.wav") {
            warp::reply::with_header(reply, "Content-Type", "audio/x-wav").into_response()
        } else if reply.path().ends_with("sample-m4b.m4b") {
            warp::reply::with_header(reply, "Content-Type", "audio/x-m4a").into_response()
        } else if reply.path().ends_with("sample-m4p.m4p") {
            warp::reply::with_header(reply, "Content-Type", "audio/x-m4a").into_response()
        } else if reply.path().ends_with("sample-6s-cors-restricted.wav") {
            println!("1{:?}", reply);
            warp::reply::with_header(reply, "Access-Control-Allow-Origin", "https://developer.mozilla.org").into_response() 
        } else if reply.path().ends_with("sample-6s-cors-allow.wav") {
            println!("1{:?}", reply);
            warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*").into_response() 
        } else {
            reply.into_response()
        }
        
    );
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
            } else if reply.path().ends_with("thumbnail_960x540.m4v") {
                println!("1{:?}", reply);
                warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*").into_response() 
            } else if reply.path().ends_with("file_example_AVI_1280_1_5MG.avi") {
                println!("1{:?}", reply);
                warp::reply::with_header(reply, "Content-Type", "video/avi").into_response()
            } else if reply.path().ends_with("sample_960x540.mp4") {
                println!("1{:?}", reply);
                warp::reply::with_header(reply, "Content-Type", "video/mpeg").into_response()
            } else if reply.path().ends_with("cors_sample_960x540.mp4") {
                println!("1{:?}", reply);
                warp::reply::with_header(reply, "Access-Control-Allow-Origin", "https://developer.mozilla.org").into_response()
            } else {
                println!("else {:?}", reply);
                reply.into_response()
            }
        });

    
    let health_check  = warp::get()
        .and(warp::path("health")).and(warp::path::end()).and_then(health_check);
    
    let forbidden  = warp::get()
        .and(warp::path("403route")).and(warp::path::end()).and_then(forbidden_route);
    
    let mut redirect_path = envy::from_env::<EnvConfiguration>()
    .expect("Please provide BASE_URL env vars");
    redirect_path = EnvConfiguration{base_url: redirect_path.base_url.to_owned() + "images/redirect.jpeg"};
    
    let mut redirect_video_path = envy::from_env::<EnvConfiguration>()
    .expect("Please provide BASE_URL env vars");
    redirect_video_path = EnvConfiguration{base_url: redirect_video_path.base_url.to_owned() + "images/redirect_sample_960x540.mp4"};

    let mut redirect_audio_path = envy::from_env::<EnvConfiguration>()
    .expect("Please provide BASE_URL env vars");
    redirect_audio_path = EnvConfiguration{base_url: redirect_audio_path.base_url.to_owned() + "audio/sample-6s-redirect.wav"};

    let redirect_route = warp::path("will_redirect").map(move|| {
        let uri = redirect_path.base_url.parse::<Uri>().unwrap();
        print!("uri: {:?}", uri);
        warp::redirect(uri)
    });

    let redirect_video_route = warp::path("redirect_video").map(move|| {
        let uri_video = redirect_video_path.base_url.parse::<Uri>().unwrap();
        print!("uri: {:?}", uri_video);
        warp::redirect(uri_video)
    });

    let redirect_audio_route = warp::path("redirect_audio").map(move|| {
        let uri_audio = redirect_audio_path.base_url.parse::<Uri>().unwrap();
        print!("uri: {:?}", uri_audio);
        warp::redirect(uri_audio)
    });

   

    let routes = health_check.or(assets).or(index).or(audio).or(redirect_route).or(redirect_video_route).or(redirect_audio_route).or(forbidden)
        .with(cors);
        // .recover(return_error);

    warp::serve(routes).run(([0, 0, 0, 0], 8088)).await;
}
