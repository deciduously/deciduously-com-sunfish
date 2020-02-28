use crate::{blog::*, handlers::*};
use hyper::{Body, Method, Request};
use log::{info, warn};

pub async fn router(req: Request<Body>) -> HandlerResult {
    let (method, path) = (req.method(), req.uri().path());
    info!("{} {}", method, path);
    match (method, path) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => index().await,
        (&Method::GET, "/blog") => blog().await,
        (&Method::GET, "/cv") => cv().await,
        (&Method::GET, "/projects") => projects().await,
        (&Method::GET, "/main.css") => {
            string_handler(include_str!("assets/main.css"), "text/css", None).await
        }
        (&Method::GET, "/tomorrow-night.min.css") => {
            string_handler(
                include_str!("assets/tomorrow-night.min.css"),
                "text/css",
                None,
            )
            .await
        }
        (&Method::GET, "/manifest.json") => {
            string_handler(include_str!("assets/manifest.json"), "text/json", None).await
        }
        (&Method::GET, "/highlight.pack.js") => {
            string_handler(
                include_str!("assets/highlight.pack.js"),
                "application/javascript",
                None,
            )
            .await
        }
        (&Method::GET, "/robots.txt") => {
            string_handler(include_str!("assets/robots.txt"), "text", None).await
        }
        (&Method::GET, path_str) => {
            // Otherwise...
            // is it an image?
            if let Some(ext) = path_str.split('.').nth(1) {
                match ext {
                    "png" | "svg" => image(path).await,
                    _ => four_oh_four().await,
                }
            } else {
                // No extension... is is a published blog post?
                blog_handler(path_str).await
            }
        }
        _ => {
            warn!("{}: 404!", path);
            four_oh_four().await
        }
    }
}
