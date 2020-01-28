// handlers.rs
// Web route handlers and router

use crate::templates::*;
use askama::Template;
use hyper::{Body, Method, Request, Response, StatusCode};
use std::{convert::Infallible, str::FromStr};

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

pub async fn router(req: Request<Body>) -> HandlerResult {
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => index().await,
        (&Method::GET, "/cv") => cv().await,
        (&Method::GET, "/robots.txt") => robots().await,
        _ => four_oh_four().await,
    }
}
