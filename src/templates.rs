// templates.rs
// Typed structs for each template in /templates/

use crate::{
    blog::{LinkInfo, LINKINFO},
    config::{CVDATA, NAV},
    types::*,
};
use askama::Template;
use lazy_static::lazy_static;

lazy_static! {}

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
    header: &'static CvHeader,
    img_dim: usize,
}

impl Default for IndexTemplate {
    fn default() -> Self {
        Self {
            links: &NAV,
            header: &CVDATA.header,
            img_dim: 32,
        }
    }
}

#[derive(Template)]
#[template(path = "blog.html")]
pub struct BlogTemplate {
    links: &'static [Hyperlink],
    posts: &'static [LinkInfo],
}

impl Default for BlogTemplate {
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
    cv: &'static CV,
    img_dim: usize,
    links: &'static [Hyperlink],
}

impl Default for CvTemplate {
    fn default() -> Self {
        Self {
            cv: &CVDATA,
            img_dim: 32,
            links: &NAV,
        }
    }
}
