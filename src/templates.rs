// templates.rs
// Typed structs for each template in /templates/

use crate::types::*;
use askama::Template;
use std::str::FromStr;

#[derive(Default, Template)]
#[template(path = "skel.html")]
pub struct SkelTemplate {
    links: Vec<Hyperlink>,
}

#[derive(Default, Template)]
#[template(path = "404.html")]
pub struct FourOhFourTemplate {
    links: Vec<Hyperlink>,
}

#[derive(Default, Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    links: Vec<Hyperlink>,
}

#[derive(Default, Template)]
#[template(path = "cv.html")]
pub struct CvTemplate {
    cv: CV,
    img_dim: usize,
    links: Vec<Hyperlink>,
}

impl CvTemplate {
    fn new(s: &str) -> Result<Self, toml::de::Error> {
        Ok(Self{ cv: toml::from_str(&s)?, img_dim: 32, links: Vec::default() })
    }
}

impl<'a> FromStr for CvTemplate {
    type Err = toml::de::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(CvTemplate::new(s)?)
    }
}
