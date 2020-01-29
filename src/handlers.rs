// handlers.rs
// Web route handlers and router

use crate::templates::*;
use askama::Template;
use hyper::{header, Body, Method, Request, Response, StatusCode};
use log::{info, warn};
use std::{convert::Infallible, path::PathBuf, str::FromStr};

type HandlerResult = Result<Response<Body>, Infallible>;

async fn cv() -> HandlerResult {
    let markup =
        CvTemplate::from_str(include_str!("assets/cv.toml")).expect("Should parse cv.toml");
    Ok(Response::new(Body::from(
        markup.render().expect("Should render markup"),
    )))
}

async fn four_oh_four() -> HandlerResult {
    let markup = FourOhFourTemplate::default();
    Ok(Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(Body::from(markup.render().expect("Should render markup")))
        .unwrap())
}

async fn index() -> HandlerResult {
    let markup = IndexTemplate::default();
    Ok(Response::new(Body::from(
        markup.render().expect("Should render markup"),
    )))
}

async fn robots() -> HandlerResult {
    Ok(Response::new(Body::from(include_str!("assets/robots.txt"))))
}

async fn image(path_str: &str) -> HandlerResult {
    let path_buf = PathBuf::from(path_str);
    let file_name = path_buf.file_name().unwrap().to_str().unwrap();
    let ext = path_buf.extension().unwrap().to_str().unwrap();

    match ext {
        "svg" => {
            // build the response
            let body = {
                let xml = match file_name {
                    "dev-badge.svg" => include_str!("assets/images/dev-badge.svg"),
                    "linkedin-icon.svg" => include_str!("assets/images/linkedin-icon.svg"),
                    "github.svg" => include_str!("assets/images/github.svg"),
                    _ => "",
                };
                Body::from(xml)
            };
            Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header(header::CONTENT_TYPE, "image/svg+xml")
                    .body(body)
                    .unwrap(),
            )
        }
        _ => four_oh_four().await,
    }
}

pub async fn stylesheet() -> HandlerResult {
    Ok(Response::new(Body::from(include_str!("assets/main.css"))))
}

pub async fn router(req: Request<Body>) -> HandlerResult {
    let (method, path) = (req.method(), req.uri().path());
    info!("{} {}", method, path);
     match (method, path) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => {index().await},
        (&Method::GET, "/cv") => cv().await,
        (&Method::GET, "/main.css") => stylesheet().await,
        (&Method::GET, "/robots.txt") => robots().await,
        (&Method::GET, path_str) => image(path_str).await,
        _ => {
            warn!("{}: 404!", path);
            four_oh_four().await},
    }
}
