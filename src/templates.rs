// templates.rs
// Typed structs for each template in /templates/

use crate::{blog::{LINKINFO, LinkInfo}, config::NAV, types::*};
use askama::Template;
use lazy_static::lazy_static;
use std::str::FromStr;

lazy_static! {

}

#[derive(Template)]
#[template(path = "skel.html")]
pub struct SkelTemplate {
    links: &'static [Hyperlink],
}

impl Default for SkelTemplate {
    fn default() -> Self {
        Self { links: &NAV }
    }
}

#[derive(Template)]
#[template(path = "404.html")]
pub struct FourOhFourTemplate {
    links: &'static [Hyperlink],
}

impl Default for FourOhFourTemplate {
    fn default() -> Self {
        Self { links: &NAV }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    links: &'static [Hyperlink],
    posts: &'static [LinkInfo],
}

impl Default for IndexTemplate {
    fn default() -> Self {
        Self {
            links: &NAV,
            posts: &LINKINFO.published,
        }
    }
}

#[derive(Template)]
#[template(path = "cv.html")]
pub struct CvTemplate {
    cv: CV,
    img_dim: usize,
    links: &'static [Hyperlink],
}

impl CvTemplate {
    fn new(s: &str) -> Result<Self, toml::de::Error> {
        Ok(Self {
            cv: toml::from_str(&s)?,
            img_dim: 32,
            links: &NAV,
        })
    }
}

impl<'a> FromStr for CvTemplate {
    type Err = toml::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CvTemplate::new(s)?)
    }
}