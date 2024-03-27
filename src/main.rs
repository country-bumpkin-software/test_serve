use askama::Template;
use serde::Deserialize;
use std::env;
use std::{thread, time};
use warp::http::Uri;
use warp::reply::html;
use warp::reply::Response;
use warp::{http::Method, http::StatusCode, reply::Reply, Filter};
use warp::{reply, Rejection};
extern crate blob;
use blob::Blob;
use std::str::FromStr;

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
    Ok(warp::reply::with_status(
        "Service is running",
        StatusCode::OK,
    ))
}

async fn forbidden_route() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status(
        "You entered the forbidden realm",
        StatusCode::FORBIDDEN,
    ))
}

pub async fn home_handler() -> WebResult<impl Reply> {
    let path = env::var("BASE_URL");
    let template = BaseUrl {
        site_url: &path.unwrap(),
    };
    let res = template.render().unwrap();
    Ok(html(res))
}

#[tokio::main]
async fn main() {
    let path = env::var("BASE_URL");

    let body = BaseUrl {
        site_url: &path.clone().unwrap(),
    }; // instantiate your struct
    body.render().unwrap();
    // let body = body.render().unwrap();
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(&[Method::GET]);

    // println!("the $PATH variable at the time of compiling was: {:?}", path.unwrap());

    let index = warp::path("home")
        .and(warp::path::end())
        .and_then(home_handler);
    let audio =
        warp::path("audio")
            .and(warp::fs::dir("audio"))
            .map(|reply: warp::filters::fs::File| {
                if reply.path().ends_with("sample-12s.mp3") {
                    warp::reply::with_header(reply, "Content-Type", "audio/mp3").into_response()
                } else if reply.path().ends_with("sample-mp4.mp4") {
                    warp::reply::with_header(reply, "Content-Type", "audio/mp4").into_response()
                } else if reply.path().ends_with("sample-m4a.m4a") {
                    warp::reply::with_header(reply, "Content-Type", "audio/mp4").into_response()
                } else if reply.path().ends_with("sample-m4r.m4r") {
                    warp::reply::with_header(reply, "Content-Type", "audio/mp4").into_response()
                } else if reply.path().ends_with("sample-m4v.m4v") {
                    warp::reply::with_header(reply, "Content-Type", "audio/mp4").into_response()
                } else if reply.path().ends_with("sample-oversized-120mb.mp4") {
                    warp::reply::with_header(reply, "Content-Type", "audio/mp4").into_response()
                } else if reply.path().ends_with("sample-12s-mpeg.mpeg") {
                    warp::reply::with_header(reply, "Content-Type", "audio/mpeg").into_response()
                } else if reply.path().ends_with("sample3.mp2") {
                    warp::reply::with_header(reply, "Content-Type", "audio/mpeg").into_response()
                } else if reply.path().ends_with("32ch-44100-bwf.wav") {
                    warp::reply::with_header(reply, "Content-Type", "audio/x-wav").into_response()
                } else if reply.path().ends_with("sample-m4b.m4b") {
                    warp::reply::with_header(reply, "Content-Type", "audio/x-m4a").into_response()
                } else if reply.path().ends_with("sample-m4p.m4p") {
                    warp::reply::with_header(reply, "Content-Type", "audio/x-m4a").into_response()
                } else if reply.path().ends_with("webm_audio.webm") {
                    warp::reply::with_header(reply, "Content-Type", "audio/webm").into_response()
                } else if reply.path().ends_with("invalid.mpeg") {
                    warp::reply::with_header(reply, "Content-Type", "audio/whoopwhoop")
                        .into_response()
                } else if reply.path().ends_with("timeout.wav") {
                    thread::sleep(time::Duration::from_millis(91000));
                    warp::reply::with_header(reply, "Content-Type", "audio/wav").into_response()
                } else if reply.path().ends_with("sample-6s-cors-restricted.wav") {
                    println!("1{:?}", reply);
                    warp::reply::with_header(
                        reply,
                        "Access-Control-Allow-Origin",
                        "https://developer.mozilla.org",
                    )
                    .into_response()
                } else if reply.path().ends_with("sample-6s-cors-allow.wav") {
                    println!("1{:?}", reply);
                    warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*")
                        .into_response()
                } else {
                    reply.into_response()
                }
            });
    let tiff = warp::path!("images" / "tiff")
        .and(warp::fs::dir("images/tiff"))
        .map(|reply: warp::filters::fs::File| {
            if reply.path().ends_with("shapes_900kb.tiff") {
                println!("1{:?}", reply);
                warp::reply::with_header(reply, "Content-Type", "image/tiff").into_response()
            } else if reply.path().ends_with("shapes_com_15kb.tif") {
                println!("1{:?}", reply);
                warp::reply::with_header(reply, "Content-Type", "image/tif").into_response()
            } else if reply.path().ends_with("shapes_com_15kb.tiff") {
                println!("1{:?}", reply);
                warp::reply::with_header(reply, "Content-Type", "image/tiff").into_response()
            } else if reply.path().ends_with("23mb_no_compres.tiff") {
                println!("1{:?}", reply);
                warp::reply::with_header(reply, "Content-Type", "image/tiff").into_response()
            } else if reply.path().ends_with("25mb_no_compres.tiff") {
                println!("1{:?}", reply);
                warp::reply::with_header(reply, "Content-Type", "image/tiff").into_response()
            } else if reply.path().ends_with("shapes_com_15kb_bad_mime.tiff") {
                println!("1{:?}", reply);
                warp::reply::with_header(reply, "Content-Type", "image/tifff").into_response()
            } else {
                reply.into_response()
            }
        });
    let assets =
        warp::path("images")
            .and(warp::fs::dir("images"))
            .map(|reply: warp::filters::fs::File| {
                print!("{:?}", reply);
                if reply.path().ends_with("mp4file.mp4") {
                    println!("1{:?}", reply);
                    warp::reply::with_header(reply, "Content-Type", "video/avi").into_response()
                } else if reply.path().ends_with("HEIC_GOOD.heic") {
                    println!("1{:?}", reply);
                    warp::reply::with_header(reply, "Content-Type", "image/heic").into_response()
                } else if reply.path().ends_with("cors.jpeg") {
                    println!("1{:?}", reply);
                    warp::reply::with_header(
                        reply,
                        "Access-Control-Allow-Origin",
                        "https://developer.mozilla.org",
                    )
                    .into_response()
                } else if reply.path().ends_with("cors_allowed.jpeg") {
                    println!("1{:?}", reply);
                    warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*")
                        .into_response()
                } else if reply.path().ends_with("lottie.json") {
                    println!("{:?}", reply);
                    warp::reply::with_header(reply, "Content-Type", "application/json")
                        .into_response()
                } else if reply.path().ends_with("jokely_lottie.json") {
                    println!("{:?}", reply);
                    warp::reply::with_header(reply, "Content-Type", "application/json")
                        .into_response()
                } else if reply.path().ends_with("lottie_optimised.json") {
                    println!("{:?}", reply);
                    warp::reply::with_header(reply, "Content-Type", "application/json")
                        .into_response()
                } else if reply.path().ends_with("invalid_lottie.json") {
                    println!("{:?}", reply);
                    warp::reply::with_header(reply, "Content-Type", "application/json")
                        .into_response()
                } else if reply.path().ends_with("empty.json") {
                    println!("{:?}", reply);
                    warp::reply::with_header(reply, "Content-Type", "application/json")
                        .into_response()
                } else if reply.path().ends_with("thumbnail_960x540.m4v") {
                    println!("1{:?}", reply);
                    warp::reply::with_header(reply, "Access-Control-Allow-Origin", "*")
                        .into_response()
                } else if reply.path().ends_with("file_example_AVI_1280_1_5MG.avi") {
                    println!("1{:?}", reply);
                    warp::reply::with_header(reply, "Content-Type", "video/avi").into_response()
                } else if reply.path().ends_with("sample_960x540.mp4") {
                    println!("1{:?}", reply);
                    warp::reply::with_header(reply, "Content-Type", "video/mpeg").into_response()
                } else if reply.path().ends_with("17mb_jpeg.txt") {
                    println!("1{:?}", reply);
                    warp::reply::with_header(reply, "Content-Type", "html/text").into_response()
                } else if reply.path().ends_with("heic.txt") {
                    println!("1{:?}", reply);
                    warp::reply::with_header(reply, "Content-Type", "html/text").into_response()
                } else if reply.path().ends_with("cors_sample_960x540.mp4") {
                    println!("1{:?}", reply);
                    warp::reply::with_header(
                        reply,
                        "Access-Control-Allow-Origin",
                        "https://developer.mozilla.org",
                    )
                    .into_response()
                } else if reply.path().ends_with("timeout.jpeg") {
                    thread::sleep(time::Duration::from_millis(91000));
                    warp::reply::with_header(reply, "Content-Type", "image/jpeg").into_response()
                } else if reply.path().ends_with("timeout.mp4") {
                    thread::sleep(time::Duration::from_millis(91000));
                    warp::reply::with_header(reply, "Content-Type", "video/mp4").into_response()
                } else {
                    println!("else {:?}", reply);
                    reply.into_response()
                }
            });

    let health_check = warp::get()
        .and(warp::path("health"))
        .and(warp::path::end())
        .and_then(health_check);

    let forbidden = warp::get()
        .and(warp::path("403route"))
        .and(warp::path::end())
        .and_then(forbidden_route);

    let mut redirect_path =
        envy::from_env::<EnvConfiguration>().expect("Please provide BASE_URL env vars");
    redirect_path = EnvConfiguration {
        base_url: redirect_path.base_url.to_owned() + "images/redirect.jpeg",
    };

    let mut redirect_video_path =
        envy::from_env::<EnvConfiguration>().expect("Please provide BASE_URL env vars");
    redirect_video_path = EnvConfiguration {
        base_url: redirect_video_path.base_url.to_owned() + "images/redirect_sample_960x540.mp4",
    };

    let mut redirect_audio_path =
        envy::from_env::<EnvConfiguration>().expect("Please provide BASE_URL env vars");
    redirect_audio_path = EnvConfiguration {
        base_url: redirect_audio_path.base_url.to_owned() + "audio/sample-6s-redirect.wav",
    };

    let redirect_route = warp::path("will_redirect").map(move || {
        let uri = redirect_path.base_url.parse::<Uri>().unwrap();
        print!("uri: {:?}", uri);
        warp::redirect(uri)
    });

    let redirect_video_route = warp::path("redirect_video").map(move || {
        let uri_video = redirect_video_path.base_url.parse::<Uri>().unwrap();
        print!("uri: {:?}", uri_video);
        warp::redirect(uri_video)
    });

    let redirect_audio_route = warp::path("redirect_audio").map(move || {
        let uri_audio = redirect_audio_path.base_url.parse::<Uri>().unwrap();
        print!("uri: {:?}", uri_audio);
        warp::redirect(uri_audio)
    });

    let png_txt = include_str!("../dataurl/png.in");
    let dino_png_txt = include_str!("../dataurl/dino_png.in");
    let data_url_small_jpeg_txt = include_str!("../dataurl/small_jpeg.in");
    let data_url_svg_text = include_str!("../dataurl/svg.in");
    let dog_svg_text = include_str!("../dataurl/dog_svg.in");
    let zog_svg_text = include_str!("../dataurl/zog_svg.in");

    let dataUrlPng = warp::path("data_url_png").map(move || warp::reply::html(png_txt));
    let dataUrlDinoPng =
        warp::path("data_url_dino_png").map(move || warp::reply::html(dino_png_txt));
    let data_url_small_jpeg =
        warp::path("data_url_small_jpeg").map(move || warp::reply::html(data_url_small_jpeg_txt));
    let data_url_svg = warp::path("data_url_svg").map(move || warp::reply::html(data_url_svg_text));
    let data_url_dog_svg =
        warp::path("data_url_dog_svg").map(move || warp::reply::html(dog_svg_text));
    let data_url_zog_svg =
        warp::path("data_url_zog_svg").map(move || warp::reply::html(zog_svg_text));
    let uri_svg1 = warp::path("uri_svg1").map(move || warp::reply::html("data:image/svg+xml,%3Csvg%20height%3D%22150%22%20width%3D%22400%22%3E%0A%20%20%3Cdefs%3E%0A%20%20%20%20%3ClinearGradient%20id%3D%22grad1%22%20x1%3D%220%25%22%20y1%3D%220%25%22%20x2%3D%22100%25%22%20y2%3D%220%25%22%3E%0A%20%20%20%20%20%20%3Cstop%20offset%3D%220%25%22%20style%3D%22stop-color%3Argb(255%2C255%2C0)%3Bstop-opacity%3A1%22%20%2F%3E%0A%20%20%20%20%20%20%3Cstop%20offset%3D%22100%25%22%20style%3D%22stop-color%3Argb(255%2C0%2C0)%3Bstop-opacity%3A1%22%20%2F%3E%0A%20%20%20%20%3C%2FlinearGradient%3E%0A%20%20%3C%2Fdefs%3E%0A%20%20%3Cellipse%20cx%3D%22200%22%20cy%3D%2270%22%20rx%3D%2285%22%20ry%3D%2255%22%20fill%3D%22url(%23grad1)%22%20%2F%3E%0A%20%20Sorry%2C%20your%20browser%20does%20not%20support%20inline%20SVG.%0A%3C%2Fsvg%3E"));
    let uri_svg2 = warp::path("uri_svg2_html").map(move || warp::reply::html("data:image/svg+xml,%3C!DOCTYPE%20html%3E%0A%3Chtml%3E%0A%3Cbody%3E%0A%0A%3Csvg%20height%3D%22150%22%20width%3D%22400%22%3E%0A%20%20%3Cdefs%3E%0A%20%20%20%20%3ClinearGradient%20id%3D%22grad1%22%20x1%3D%220%25%22%20y1%3D%220%25%22%20x2%3D%22100%25%22%20y2%3D%220%25%22%3E%0A%20%20%20%20%20%20%3Cstop%20offset%3D%220%25%22%20style%3D%22stop-color%3Argb(255%2C255%2C0)%3Bstop-opacity%3A1%22%20%2F%3E%0A%20%20%20%20%20%20%3Cstop%20offset%3D%22100%25%22%20style%3D%22stop-color%3Argb(255%2C0%2C0)%3Bstop-opacity%3A1%22%20%2F%3E%0A%20%20%20%20%3C%2FlinearGradient%3E%0A%20%20%3C%2Fdefs%3E%0A%20%20%3Cellipse%20cx%3D%22200%22%20cy%3D%2270%22%20rx%3D%2285%22%20ry%3D%2255%22%20fill%3D%22url(%23grad1)%22%20%2F%3E%0A%20%20Sorry%2C%20your%20browser%20does%20not%20support%20inline%20SVG.%0A%3C%2Fsvg%3E%0A%0A%3C%2Fbody%3E%0A%3C%2Fhtml%3E"));
    let data_url_tiff = warp::path("tiff_dataurl").map(move || warp::reply::html("data:image/tiff;base64,SUkqAPguAAB4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3OsQnCQBQA0JQBkeyTLZxAcAJdIeACdnbBCVIFN3AEG6u0qYIEUt1lh2t+894Er6oAAIBoh/rYrNt/iX4AAOVO7eU6fPpHzilFXwCAMvfz6/0cu9s0/77RFyDGDk6DFAF4nGNgGAWjYBQMZcDIyMS0pur8m5nbm4t2nVuzYKDdMwpGwSgYBaNgFIwC0oGMiJL63IJ9N7aeXjZz0saqjIF2zygYBaNgFIyCUTAKSAcuhsFxpcG9C+8+v3Yha6qX4UC7ZxSMglEwCkbBKBgFpINcv5ZpPmYxmf////sX0KzD9+PXt68D7aZRMApGAf0BAJmVJAF4nGNgGAWjYBQMZTA1a8tZFSkdIxC7dG64w6X7Jw8OtJtGwSgYBaNgFIyC4QLk2di0eJmYhGhpBxsrJ0db6ZmdjEzMTCD+lr090/cfn7OMlnbiAtd//jzx9///PwNh9ygYBaNgFIyCUUAr0CQhsdGKm9uPlnb8F9dl+Oc1Ac5nfHiEgWlfPS2txAn87t/n+/bv3+cBsXwUjIJRMApGwSigEaBLfa4TxvDPNB0h8O0tA/PKcJAMLa3FCkbr81EwCgYWAACujTUBeJzt2CEKAlEUheHznvGBTZjoOgyCG5go7sIlCLMJtyBiMZjFatY8Zo2izaeObYaZeEcY/i/ecuIPVwJgIUuS7SiE1HIjThZ6D8elW289kx43y9laaZ73nzHeWx8GAMBQGz1/TVdSGJRufp/JXQ6Ws7XoOQCgi8x7/u34r+cV7ryRPy7NZpvQcwBAF1n3vPizF//2Knc9ye/mVrON6DnwXx/0TDYBeJzt1SEOwkAQheHprkA0wSDoAbB1KI5AQi0GTcI1MFiCw3OBgkVVcQMSEtIToEgQhO6CZ1cOJM3/2TeZJ58IAA3LLCtHaVpo/XfDufh8+h00T7G7yefgpVUdVNR19+Hc/aelAAAoU9/z8Vp8Pw9mZr+Q5HbRqg5izwEAbaS658ZKMzuI2E44Pm0kOZcq1THsOQCgjTT33PcG4optNE+uRzHVSqM6ij0H/usNdG05AXic7dUhCsJwGIbxdxtYhlpXbAavoGA1DrQKXsEoGD3JQLAJmj2BSW+woheYIBj2nxfYZvFTGM8vf3xvfCQAFjZRdByFYWzxuxjEcsNl9UF2V7BfWExXitO083Tu8dNRAACMWfbcjVcq+pPam2A3lV6ZxXwpeg4AaCLLnuezROr2am/801re7WwxX4qeAwCayKznrbby+eHjmXfdyr8kX5+vQs+B/3oDirw2AXic7dgxCsJAEAXQvyvYBCwloiDeQAsRBEELLXMXKzsLC/EAXsRK8AaKjb2QRsFGEMQgBncV6wRXEhMI/9WzM9N9dgAi+oeJbS/aluXE3VeXm1CD2dc6cdxCrkZxjw/luG7BU+qa2EAiIqIEBOa5VYQu1SP11ZUWdK37vfBxg1zPI83C/QJx2BiVMs+JiCiLAvNcSKjeGLraSWmrH/ge5HIIcd4blTPPiYgoi0Lv7bk8VH/6/qc3UtjK0NP/3OrFaWf8hHlOlK4XiXBBAXic7dc7CsJAGATg2QVRDGoKlTTaegdPIAq5kmCrhdhJLiB2dlZWWmmnnUXAKwh2ybrpNS/IBsJ8zV8sA9MNCxBRERaOcxhblvvzsdZEOFlBdUeGW6WgFORpDvG6ZIq5vt/+hOG7oFZERESliN3zSMNGMF0DnYHBVsnkeQnxPGbOcc+JiKiKEvc8YvURzDb69gy1iievW4jHPleWe05ERFWUas8j9lD/0/Wm11sGWv0n7jvIm5c7zz0nKtcXt443AXic7dUtC8JQGAXgcy8LwsAmDJtNs0nTimBxv0VsgmnBpD/GtGAxaDdrGCaTSVhQdq/eODS4j4swzpPe8HI47QBEZEPoeZuh6wa//OpWD2q8ApyG7VpfiVMEeVibJoUzgjhuJkrdq2tFRET0f3n23NDtPtRoCUjHZq0P4rKH3IWASkvlcM+JiKiO8u65oTs+lL94X8JSqyxxPUJu50D6KJ3FPSciojoqsueG7k6gBlMblTLE7QwZzYBnUkke95zov16CCz8BeJzt1yELwkAYxvFnm0EQLJYVk0mLYFEGFjEZFswWq0GLwarfSETxi4hRBIOiYTgZE+TOW9LsnSjj+fV7eNt/A4joG2auO/dyOf+Tt7Lag6j1TZ/0cj3AWY6AODA26e92+UiI0NggERHRH9DpOWBB1AeQla7ZoxLRRbV8CNxORmfZcyIiSiO9niuWanpzAllqmzvqHsJZqf/yYG9s0hFZ9fVho7PfsOdERJQ62j1P2BmI1hSy2NA/6BHDXo9hnbf6W28KcRmWdOAdF+w50Q89AaBxQwF4nGNgGAWjYBTQAjRJSGy04ub2o9ggFnaGv0ELGRi4RSkyhulwJwPjnV0UOwcdKH/wZWBkYGbQet/P9+3fv89Ut2AUjIJRMApGwSgYQEC1+pyJmeFv9GZwvU6RMccnMTDe2Eixc9CB8csiBqb/TAyS37NH6/NRMApGwSgYBcMOUKs+/y+kzPDPfxbF7gH1zUF9dGoCpv+sDA5PeoCOZGQQ/xsq+frftxdUtWAUjIJRMApGwSgYYEC1+lzdh+GfVSHlDvr0hIF5bTzl5iABwR+qDEav8sFs37/V/lv+ndtEVQtGwSgYBUQDANlZPgF4nO3XvWrCYBQG4DcJdonQuIUOBQV7AW2nIma0IkgFt95Dl2J/1k4l9B5KHMQ1q6Pi5KBgL6EO7Ra6JI3J1yZbR/EUMrzP8v0M7znb4QBE9B+ebNu/MM3uvjlpYwBVv5RoCcboCoi+RLIy1aCNWtDJ78/p2H1MvHuxcCIiogKQmudJ7xWwjiVagj55gLZZiGRlTj9vUAlP8vtMrefN7V1DLJyIiKgAROb5QRnJtS/UEaCthtCXnkiWrgw47y+/Zyl/R4gjK+5bIb5DkQJEREQFIDHP1dEZ0pYr1VK+m2c7uoTDqIrzj9s/f8524EzV21SkABHt5AcgT0EBeJxjYBgFo2AU0AI0SUhstOLm9qPEjP8GsQz/DBOo5CIg+PWZgXlZINDg/xQbJf/JlUHlgz+KWO3fRbUt/5a3UGz4KBgFo2AUjIJRMEgANerzfy6tDP9lLQgr/P2dgYGVkygzmdclMDB8fEyJs8DA4HUWg/B3LRSx3f/P7Xb7U+1GseGjYBSMglEwCkbBIAGU1+eMDH+j1jEwsPPhV/b9PQPztnyGf7rhDP/VvAmaynS4k4Hxzi7ynQV2GROD/ZMuBuZ/HCjiXxl+fBX8HSr4m+HPb4osGAWjYBSMglEwCgYJoLg+55Nm+Bu8CL+a398YmLYVMjC+uwOsZJkY/jnWMfyXt8WrhfHGJgam4xPJdhYI8P6SZTB7UY5VzvxPgfmp/zdPUWTBKBgFo4BkAAA8GlIBeJzt1U8oBFEcB/Dvm2kzmQNZq3XYkyKlLTcpSlLCSil7cXCSi3Ja5UIuS+sgB62DmxMu2Bz24KKU3HBaLqs9+NOmXSF23nsmF2lnWDOpOfw+c5h6v/f7vd/tCxBC/sNiMLjXqevDTvtlUy9E95z9Bf4OJT0Ldnv+dab6IPrikI3ttm0sfwVlf8rpWp9CTz1ofhy1rMX4ZiwhdhOuHiCEEEI8wm2ei45pyNYR66IUUI7mwW5Oymu+aoj+Fcj6FttedWsIMN6crobwwyQCr2HLWkqepiLGQsTxcEIIIcRDXOd5ZN02k5XjZbDrtH2zVgs+sArUhKz7D2fA7i4c7cXMryu3BJ/QLesFPBf8pTE/h+COHiCEEEI8xFWeq1Xg4wdm8KplJeUsCXa58/sMvQF8cM38ByxmbJgztqEKDW35CeilYMWrMSjQjLof72TlffYveZ5BLhM14tEiXooVL0II+eYDyidoAXic7dU9SEJRFAfw/3voM1BL+lBxCHKIwCEqaGjJQcUMaqmlqaCloaFRU+gD2gpq7WurocmlpaEoAhcpHaSMooIQskSpHMr3XupgSBB2H9rQ+d3l3nPvPedsByCEVMOC2Rzs02qHWP7KRhukwbVvcS66Cz68UXkiQytEzyqgqS/Pc3sM/nC+uBdEPboep6H7sLC0qlhMvo85cl5HAqnEnzRACCGE/EDRPLeNQOqdKotx8X3wpyuF29/laumA5F4GVHVfwWwS8egknpoFpA1qiGo9ZvbGYH0wsrTLLCLfRJw5nzOJTLKmhQkhhJAKKZnnkj0Auc1eOnN3J+CPFvMXIlMvsqUHknMJ4FWlmK97DmkhXTo3vGqw5feg87I2Mz0sX4VduVlXCi+pmhQkhBBCGCiZ5+LoDqAzFfdc4hz8gTcffGfq47lJQHDYBNHaj4nrcXD5VbDevo2zxkjZW11WwGZgAD0xM1OtSoXki5A753dn8JapaiFC/pFPVrVwAXic7dhNKINxHAfw77NnEuPipbalSbnIUVLbJGMYMRktObjIkXJy8jKFk8RBcXHlQFPaQVukJSUk5WB5yRymlDDbeJ49th1tw382p9/nufx7nt/L//atByCEZIJNqbRrFYoO5sbcQojW9diRe7yEzDECfLwxj3nL5eEwFcNlKIIg52Lvan069F73xM47ahc2NVvx6wNZWJloQc2Zmnnnb+xL5/ttwljbCwIvGVlACCGEpFGqeS5pdAg32IBnL/jtYSD4xNYfiW63rgAbXUr4FXzcd9N9E9rvWuHJv8Jc5ULCGTkhOZZszdAfl7Be/1tO6dRpFibNfgT9aR1MCCGEZEiqeR6uGoBUboxk+RDw6mPqFXkOa1YV9uoKk9Zwkaf7phP6By1GqkchcmLCuux3HovTRhgOS5nukIxDOnJYhClLAO+BtAwkhBBC/kHKeV4/DtnJKvB0y9QX/b++PKjBRUXej7XRTO/39MGp2sWdwpu0LkuQYX62Ec3uMqa7fGUPH9it4ow1hI/QnwYRQpL6BFFkdwF4nGNgGAWjYBTQAjRJSGy04ub2I1XffwEFBsYPD0jS806IlWFiviLDCwl2ovUw/2dmEP4pxPCK4zV+dX+ZGHq7HRl8DqqQ5CYYWP3v8Orov13Rvxn+/CbLgFEwCkbBKBgFo2AAAbn1OangJzsTQ1eZMsMTGQ6a2cH8j5Fheak/g9E1cZL0nfx/86TNn2KbPwx//9DIaaNgFIyCUTAKRgFNAT3q8/+MDAxzUuQYzpjw09IaMNiVGs6g9ESAJD13/z+/q/InibyO/SgYBaNgFIyCUTAIAD3q8x0eogzrAyVoaQUYiLznZDgRFUeWXpk/MTJP/799SmUnjYJRMArQAACXf0IBeJxjYBgFo2AU0AI0SUhstOLm9qOV+bfUuBn6ipQY/jPSygYE8DysxDC5zZUsvQkKM7oX3t5YRmUnjYJRMApGwSgYBXQBtKzPQXV4e6UKw0N5TloYjwHqp1szxG7SIUvvYruLH+IPVIr+//v/D5WdNQpGwSgYBaNgFNAc0LI+P2vMzzArTY4WRmMFW7JCGDTuC5Ol947cewYj1crkzzsezqOys0bBKBgFo2AUjAKaA1rV53+ZGRkaGlQZXomxU9torEDgMzvD6fAEBsb/5JthNHXmsws5m5X///r7g3ouGwWjYBQgAwA7QzoBeJxjYBgFo2AU0AI0SUhstOLm9qO2uYdthRiWxEhT21icwPmEAsPMRnec8m2pxxn+Mv9jqJ1hjVNNds0uhmWXNxd/WHOnjxZuHAWjYBSMglEwCmgFaFWfd5YrM9xT4qK2sThB5RxLhuS1eljlGrKOMCzxvQpmR27TYmiebItV3YKAywx17juvPk7Zq0Mzh46CUTAKRsEoGAU0ACnCwu06HBw21DTzKz8r27yJ+mYMjIzUNBYvWJcfxKB3SxRF7D/Q+pq8QwwrPa6jiAfvVmfo6HdgYPyPasY15TcMflPWMjyK263y+9nXu7R28ygYBaNgFIyCUTCYAZ+nfLJoseEcetnH/Z2V4VxIIgPzP0T74R+wsi4vPsCw3vkWVj1++1UYunucUPSA6n/j1fMZ7i06W/hx7d0JNHf4KBgFIxAAAFOvVQF4nGNgGAWjYBQMFSDRZL6B20rSn1722Z6VZZhf4wXn/2X6z1BUvpdhq91dvPo8jigxTOxwZmD+ywQXS23czrCN7dz+ZyVHnGjm4FEwCkbBKBgFo2CQA0YWJjbFjd4fGNmZOellZ/FCM4bMFYZg9h+Wfwx5lXsYdlndJ0qv8wkFhimtrgysfyB1+qzQCwxdCSf+3g/YKvTv259PNHP0KBgFo2AUjIJRMIgBiziXvPxStwf0tHNFiT+DyVUJhl+sfxmya3Yx7Dd7RJJ+uzOyDNOb3RnYfzEzXNB4yRDSv4Hhceo+vV/3P12mkZNHwSgYBaNgFIyCQQ04tIQspSfZHaObfb9YGM4HJzL8Y/rPkF6/g+GI0ROyzLG6IM0ws8GDgQ3YTzdYM5/hXsMhj29nXu2ksnNHwSgY8QAAJrxYAXicY2AYBaNgFAwFwG0jFSTRYLaWXvaZX5JimFPvyZDSuJ3hpN4ziswyvSLJMLfOkyGzdifD+j2bEz/vfLSAOq4cBaNgFIyCUTAKhhbgD1DKEcnRm0wv+xI36DJcUnvNcFbrBVXMM7ghzmB0TZyh5/2KqvfLb7VTxdBRMApGwSgYBaNgiAGhRM0WwWj1anrZx/WdleEb52+qm/l8583Jb6ZcyqOqwaNgFIyCUTAKRsEQAaC6HFSnD7Q7KAUfVtzufDvnasVAu2MUjILhBgBj+TUBeJxjYBgFo2AUDAXA6yGfJFZiOHeg3UEpeDPtcsHHdXcnDrQ7RsEoGAWjYBSMgoEAXKbiHpLtltsH2h2UgpfNp8O/HHy6aqDdMQpGwSgYBaNgFAwEYFPm15ed6XhhoN1BKXhaeNjux+W3hwfaHaNgFIyCUTAKRsFAAGYBdjGFNZ4vB9odlIJH8btVfz/9emeg3TEKRsFwAwAK/yABeJztzr8KQXEYxvH3dzo/JxYDMSpWWQxcAGZiMLoA98BiYHUFSkY26swmyuAGyCCJlD/Jcc7BJDYp3uj51Ls89daXCAB+giARaKYmqt8V4E55l7U11tNcz0/2xeJuAQAA4OItRurudLDI3fGunT5rLGujAncHAAAAJ2fUlwxVY3rFGt+3nZBUUsKMVa9blAfZQ3/e5u4AAAD4Fk2SdjrT6XETquLId+Kblhy6Hve4mqAVaU//4naSbDJI+XzsCy6mbUwzXY99NPfcLQD/6ApRVTMBeJxjYBgFo2AUDCbAx8XAVxDIVFAYyFxYOudv6Zwd/+Ygy7fNkLtUqfBcF1ksi9mYYRejBJzPCMTl/64z+P17xjCFSYVhNZMsw28GJvp4AAf4dvLltufVx70H1BGjYBSMglEwCkYBHYCHCaPHkjKWJcJ8DMIg/v//DP/TJv5Nu3Dv/wU7HUY7Wx1GWycjJlc+DgZuZH1vGdgYjjGKMJxmFGI4xSjMEPj/CUP6v7tw+SeMXAzpTCYMNxl56ewjBHhWdNj++6W3hwbMAaNgFIyCUTAKRgGdgLkGo/mJCSwnqG3uH2CP3ZLFheE9sN4fCPDt1Mvtz6uOew2I5aNgFIyCUTAKRgGdASMjA+P12SzX1WUY1alp7m5GCYZMZmNqGkkSeJK+3/Dn3Y8XBswBo2AUjAAAAE46TQF4nO3XoUsDUQDH8d97T4NyM5wnMkQ4BC+tzWBSUKPLA8PyNFtspgWTYvIvcHkYtuIUFEEWNrAIInIcwjYGYwx17PQ5u8Fwb2f4fdorv+9rjwcQ0X/hzMCpHk1UU65IRbl7Leawp1YwgIxy9k/6l8F5s1DbGXuYiIhoDOwEbGtKWH5L+z9nd164lYKqeAvCM9G7F7PIqzR6mIQYnZd0H76YxtDgG//ZHbSC3at02H4PjEWIiIhiVDxQxey6zDaedaNc0+XclswlbSRNNh9FAnfCwaZuYlG/4UR6OJXLRlo6/Bq+7t9ufDx0bowEiIiIYpZZlZnSoSrFfY9w9FPfVmt4Elbk2+3jer538XIW+TAR/eobtcdJAXic7dXNCgFxFMbhc/x9TCjFRik1mymWlK2lK7C3s5gsrZUbUMpi3IJkw0K5AZSPYqks1ZiapCzk8xLUyczifS7gd97dIQKAf2Im3lvBfS7LOa+3fE05TaYqijYvk2Pf7mzqolEAAACfyWc5P2qpkZFhw8sdS05SQxXoTBGxpjs8dBxr16Tn6yEWBQAA8KlEjBLrXnCtp1n34v6cU1RTJbpTQCb4+d92d2texkdLJggAAOB/0QhFnUHI0cKkeXH/wHGqqLJI63m9u6f2onpb2TORIAD87A2RCTQBeJxjYBgFo2AUDATwMWf02dzIspkcvYcu/z908ub/k+ysDOx2Oox2BsqMBuSY48zswPCQkZscrRDwn+H/5z2PF7+bd63mz+vvj8k3aBSMglEwCkbBKBhagJGRgVFWlFG2J5WpJ9SWKZQUvd9+MnwLafkTsv30/+3I5qV7MaVPy2GeBmKTYt4sJmWGuUxKDG8Z2EjRBgbfz7/e93bW1dKftz+cI1nzKBgFo2AUjIJRMIRBfzpzf7wrU7wgD4MgOfpzpv7Nmbr531RscpOzmCfn+DHlkGPuawZ2hhlMKgwLmRTwqvv34+/XbydfbPu889H8b6df7gD1z8mxbxSMglEwCkbBKBjKYGcry043Y0Y3cvWLR/wWf/WB4RU2OQc9Rof9XSz7yTUbVJc3M2ljiP999+P5t1Ovdnw58mzd93Ov9/z/9fcHuXaMglEwCmgHAHO3bAF4nO3MMUgCURzH8f97Z3DgGkKriw01NUiBbt2ot7UqNRw4SFtbLTcdeCJCODi3mRAIIgiOFkRrziqB14GEcOLde60NLveGCPt9xv+f748IAH5L3070jRNmqPapi01qsaTFtl/+mOVHTmKkun3/ovdunvefxHLthZ/BPPSCWeQHH3Ij1qqbAAAAu6hZ0Zqlc15K6pRU6a1GZLV6orXtV7e0etXkVZVd/4t8+yGyax1RU+kBAAD+G86Ipw9Y2rnijnnGzTjtKqBV4S4sDN/k8Oe9bPBy+1prM0Yszp7bEa77KNypJ6dSkozTAgAAAFHxlBe7t1pXpR28ysH4XY71PdJzRyyXPWRZlZ3MZZiZzOREpQWAv+Ubn91eAXic7dUxCoJgGMbx9yPFQjxABA2CCJ7E3cW9YzQ0BB0h6AyBeAFH929wcm/Tpfkzo6ETNLwY/98F/s/2iADQEK4lHO/+GPgSaPT7x9ynB5dqtAEA+BfRRiJ79Wy8NbFGv7Fzkx9d7iZxGn0AAJYu2ZmkPq3qbG8yzR2fTy8vrhyeMmjuAABgaYwR0928TvvLv6r2VRXnqdDeAeA3b6EZJgF4nO3OPQtBcRTH8XPuNcpDXYlkwGSSRTHZSYzegtdwMTB4GSYTMyllsbHccg0GUUaLgfTnyHtwPf4+21nO90cEAK9WzmrlQV0fvHuHupJK11Ta2oj17i0AAADfqG/q/UpOq6x2shrNZVTNa9WAlwJONu2t2FNLpoUMFyIGR1q9W8vsXk0nmwAAAL/M8JDhc7NvvZf1406EOTFs68N4iONO9GZLmRUbqng40oGZOBXjlL0T+3yhsxM9AACAfxX0U3DScU2SUU4+8+94IeNSU5VOFzo98y8AfI47LNREAXicY2AYBaNgFAwWwMjIwHh1JstVTTlGTWqau/bIv7UhLX9DqGnmKBgFo2AUjIJRMAqwA1M1RtNTk1hOUdvcX38YfklG/pZ895nhHbXNHgWjYBSMglEwCkYBKvAxZ/RZUsayhJ+bgR8mljP1b875u//P2+ky2tnqMNna6zLac3MwcCPre/uJ4e2BS/8OHLr8/9ChK/8PxbswxRcEMhXA5J+8+f/Ep+6vz8V7/y/S0z+jYBSMglEwCkbBSAWCPAyCxcHMxfkBTPkV8/5WTN38byqyfG0UU21THHMTshhoLB00pg7jg8btJ2YwTwy3ZwpvW/G3bea2fzN//GL4QS8/jIJRMAroDwDc3lABeJztzrENQEAAheF3IeLCBDeFGYyhtYIhdGYwiHl0okCISFSSi04hFP/Xvb96EoA/SmIl86r53vPM5F0ddn5zxe76Qb3fjJGxkeyyaXn7KwAAeCa1StsqaK89ThrL5ii//AQAAAAAAABAOgFCORQBeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAAAICQ/q/uCAoAAAAAAAAAGB9AAAF4nO3BAQEAAACAkP6v7ggKAAAAAAAAABgfQAABeJztwQEBAAAAgJD+r+4ICgAAAAAAAAAYH0AAAXic7cEBAQAA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Ly9ucy5hZG9iZS5jb20vcGhvdG9zaG9wLzEuMC8iCiAgICB4bWxuczp4bXBNTT0iaHR0cDovL25zLmFkb2JlLmNvbS94YXAvMS4wL21tLyIKICAgIHhtbG5zOnN0RXZ0PSJodHRwOi8vbnMuYWRvYmUuY29tL3hhcC8xLjAvc1R5cGUvUmVzb3VyY2VFdmVudCMiCiAgIHhtcDpDcmVhdGVEYXRlPSIyMDI0LTAzLTI2VDIwOjAzOjU4KzEwMDAiCiAgIHhtcDpNb2RpZnlEYXRlPSIyMDI0LTAzLTI2VDIwOjA3OjQ3KzEwOjAwIgogICB4bXA6TWV0YWRhdGFEYXRlPSIyMDI0LTAzLTI2VDIwOjA3OjQ3KzEwOjAwIgogICBwaG90b3Nob3A6RGF0ZUNyZWF0ZWQ9IjIwMjQtMDMtMjZUMjA6MDM6NTgrMTAwMCIKICAgcGhvdG9zaG9wOkNvbG9yTW9kZT0iMyIKICAgcGhvdG9zaG9wOklDQ1Byb2ZpbGU9InNSR0IiPgogICA8eG1wTU06SGlzdG9yeT4KICAgIDxyZGY6U2VxPgogICAgIDxyZGY6bGkKICAgICAgc3RFdnQ6YWN0aW9uPSJwcm9kdWNlZCIKICAgICAgc3RFdnQ6c29mdHdhcmVBZ2VudD0iQWZmaW5pdHkgRGVzaWduZXIgMiAyLjQuMCIKICAgICAgc3RFdnQ6d2hlbj0iMjAyNC0wMy0yNlQyMDowNzo0NysxMDowMCIvPgogICAgPC9yZGY6U2VxPgogICA8L3htcE1NOkhpc3Rvcnk+CiAgPC9yZGY6RGVzY3JpcHRpb24+CiA8L3JkZjpSREY+CjwveDp4bXBtZXRhPgogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgIAogICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgCiAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAgICAKICAgICAgICAgICAgICAgICAgICAgICAgICAgCjw/eHBhY2tldCBlbmQ9InciPz4AOEJJTQQlAAAAAAAQ1B2M2Y8AsgTpgAmY7PhCfgMAAaADAAEAAAD//wAAAqADAAEAAAD0AQAAA6ADAAEAAAD0AQAAAAAAAA=="));
    let routes = health_check
        .or(assets)
        .or(dataUrlPng)
        .or(dataUrlDinoPng)
        .or(data_url_small_jpeg)
        .or(data_url_svg)
        .or(data_url_dog_svg)
        .or(data_url_zog_svg)
        .or(data_url_tiff)
        .or(uri_svg1)
        .or(uri_svg2)
        .or(index)
        .or(audio)
        .or(tiff)
        .or(redirect_route)
        .or(redirect_video_route)
        .or(redirect_audio_route)
        .or(forbidden)
        .with(cors);
    // .recover(return_error);

    warp::serve(routes).run(([0, 0, 0, 0], 8088)).await;
}
