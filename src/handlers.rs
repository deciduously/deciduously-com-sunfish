// handlers.rs
// Web route handlers and router

use crate::templates::*;
use askama::Template;
use flate2::{write::ZlibEncoder, Compression};
use hyper::{header, Body, Method, Request, Response, StatusCode};
use log::{info, warn};
use std::{convert::Infallible, io::prelude::*, path::PathBuf, str::FromStr};

type HandlerResult = Result<Response<Body>, Infallible>;

/// Top-level handler that DEFLATE compresses and responds with from a &str body
/// If None passed to status, 200 OK will be returned
async fn string_handler(
    body: &str,
    content_type: &str,
    status: Option<StatusCode>,
) -> HandlerResult {
    // Compress
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(body.as_bytes()).unwrap();
    let compressed = e.finish().unwrap();
    // Return response
    Ok(Response::builder()
        .status(status.unwrap_or_default())
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_ENCODING, "deflate")
        .body(Body::from(compressed))
        .unwrap())
}

async fn cv() -> HandlerResult {
    let template =
        CvTemplate::from_str(include_str!("assets/cv.toml")).expect("Should parse cv.toml");
    let html = template.render().expect("Should render markup");
    string_handler(&html, "text/html", None).await
}

async fn four_oh_four() -> HandlerResult {
    let template = FourOhFourTemplate::default();
    let html = template.render().expect("Should render markup");
    string_handler(&html, "text/html", Some(StatusCode::NOT_FOUND)).await
}

async fn index() -> HandlerResult {
    let template = IndexTemplate::default();
    let html = template.render().expect("Should render markup");
    string_handler(&html, "text/html", None).await
}

async fn image(path_str: &str) -> HandlerResult {
    let path_buf = PathBuf::from(path_str);
    let file_name = path_buf.file_name().unwrap().to_str().unwrap();
    if let Some(ext) = path_buf.extension() {
        match ext.to_str().unwrap() {
            "svg" => {
                // build the response
                let xml = match file_name {
                    "dev-badge.svg" => include_str!("assets/images/dev-badge.svg"),
                    "favicon.svg" => include_str!("assets/images/favicon.svg"),
                    "linkedin-icon.svg" => include_str!("assets/images/linkedin-icon.svg"),
                    "github.svg" => include_str!("assets/images/github.svg"),
                    _ => "",
                };
                string_handler(xml, "image/svg+xml", None).await
            }
            _ => four_oh_four().await,
        }
    } else {
        four_oh_four().await
    }
}

pub async fn router(req: Request<Body>) -> HandlerResult {
    let (method, path) = (req.method(), req.uri().path());
    info!("{} {}", method, path);
    match (method, path) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => index().await,
        (&Method::GET, "/cv") => cv().await,
        (&Method::GET, "/main.css") => {
            string_handler(include_str!("assets/main.css"), "text/css", None).await
        }
        (&Method::GET, "/manifest.json") => {
            string_handler(include_str!("assets/manifest.json"), "text/json", None).await
        }
        (&Method::GET, "/robots.txt") => {
            string_handler(include_str!("assets/robots.txt"), "text", None).await
        }
        (&Method::GET, path_str) => {
            // Otherwise...
            // is it an svg?
            if let Some(ext) = path_str.split('.').nth(1) {
                match ext {
                    "svg" => image(path).await,
                    _ => four_oh_four().await,
                }
            } else {
                // No extension... is is a blog post?
                // TODO
                four_oh_four().await
            }
        }
        _ => {
            warn!("{}: 404!", path);
            four_oh_four().await
        }
    }
}
