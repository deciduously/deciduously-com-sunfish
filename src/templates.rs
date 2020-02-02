// templates.rs
// Typed structs for each template in /templates/

use crate::types::*;
use askama::Template;
use lazy_static::lazy_static;
use std::str::FromStr;

type HeaderLinks = Vec<Hyperlink>;

lazy_static! {
    static ref NAV: Vec<Hyperlink> = vec![
        Hyperlink::new("deciduously.com", "/"),
        Hyperlink::new("Resume/CV", "/cv"),
        Hyperlink::new("Projects", "/projects"),
    ];
}

#[derive(Template)]
#[template(path = "skel.html")]
pub struct SkelTemplate {
    links: HeaderLinks,
}

impl Default for SkelTemplate {
    fn default() -> Self {
        Self {
            links: NAV.to_vec(),
        }
    }
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct FourOhFourTemplate {
    links: HeaderLinks,
}

impl Default for FourOhFourTemplate {
    fn default() -> Self {
        Self {
            links: NAV.to_vec(),
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    links: HeaderLinks,
    posts: Vec<Hyperlink>,
}

impl Default for IndexTemplate {
    fn default() -> Self {
        Self {
            links: NAV.to_vec(),
            posts: vec![Hyperlink::new("test", "#")],
        }
    }
}

#[derive(Template)]
#[template(path = "cv.html")]
pub struct CvTemplate {
    cv: CV,
    img_dim: usize,
    links: HeaderLinks,
}

impl CvTemplate {
    fn new(s: &str) -> Result<Self, toml::de::Error> {
        Ok(Self {
            cv: toml::from_str(&s)?,
            img_dim: 32,
            links: NAV.to_vec(),
        })
    }
}

impl<'a> FromStr for CvTemplate {
    type Err = toml::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CvTemplate::new(s)?)
    }
}
