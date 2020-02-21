// handlers.rs
// Web route handlers and router

use crate::templates::*;
use askama::Template;
use flate2::{write::ZlibEncoder, Compression};
use hyper::{header, Body, Response, StatusCode};
use std::{io::prelude::*, path::PathBuf};

pub type HandlerResult = Result<Response<Body>, anyhow::Error>;

pub async fn blog() -> HandlerResult {
    let template = BlogTemplate::default();
    let html = template.render()?;
    string_handler(&html, "text/html", None).await
}

pub async fn cv() -> HandlerResult {
    let template = CvTemplate::default();
    let html = template.render()?;
    string_handler(&html, "text/html", None).await
}

pub async fn four_oh_four() -> HandlerResult {
    let template = FourOhFourTemplate::default();
    let html = template.render()?;
    string_handler(&html, "text/html", Some(StatusCode::NOT_FOUND)).await
}

pub async fn index() -> HandlerResult {
    let template = IndexTemplate::default();
    let html = template.render()?;
    string_handler(&html, "text/html", None).await
}

pub async fn image(path_str: &str) -> HandlerResult {
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

/// Top-level handler that DEFLATE compresses and responds with from a &str body
/// If None passed to status, 200 OK will be returned
pub async fn string_handler(
    body: &str,
    content_type: &str,
    status: Option<StatusCode>,
) -> HandlerResult {
    // Compress
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(body.as_bytes())?;
    let compressed = e.finish()?;
    // Return response
    Ok(Response::builder()
        .status(status.unwrap_or_default())
        .header(header::CONTENT_TYPE, content_type)
        .header(header::CONTENT_ENCODING, "deflate")
        .body(Body::from(compressed))?)
}

pub async fn html_str_handler(body: &str) -> HandlerResult {
    string_handler(body, "text/html", None).await
}
