// this module was auto-generated by build.rs
use crate::{
    config::NAV,
    handlers::{four_oh_four, string_handler, HandlerResult},
    types::Hyperlink,
};
use askama::Template;
use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy)]
pub struct LinkInfo {
    pub id: usize,
    pub url_name: &'static str,
    pub title: &'static str,
}

#[derive(Debug, Default)]
pub struct BlogLinkInfo {
    pub drafts: Vec<LinkInfo>,
    pub published: Vec<LinkInfo>,
}

lazy_static! {
    pub static ref LINKINFO: BlogLinkInfo = {
        let mut ret = BlogLinkInfo::default();
        ret.drafts.push(LinkInfo {
            id: 3,
            title: "I Scrapped My Stencil Project And Wrote A Static Site Instead",
            url_name: "deciduously-com",
        });
        ret.published.push(LinkInfo {
            id: 0,
            title: "Thirty Green Squares",
            url_name: "green-squares",
        });
        ret.published.push(LinkInfo {
            id: 1,
            title: "That About Wraps It Up For Actix-Web",
            url_name: "actix-wrap",
        });
        ret.published.push(LinkInfo {
            id: 2,
            title: "C++ Template Specialization - Syntax Note",
            url_name: "cpp-template-specialization",
        });
        ret.published.push(LinkInfo {
            id: 4,
            title: "deciduously",
            url_name: "deciduously",
        });
        ret.published.push(LinkInfo {
            id: 5,
            title: "No More Tears, No More Knots: Arena-Allocated Trees in Rust",
            url_name: "rust-arena-trees",
        });
        ret.published.push(LinkInfo {
            id: 6,
            title: "Use Multi-Stage Docker Builds For Statically-Linked Rust Binaries",
            url_name: "multi-stage-docker",
        });
        ret
    };
}

#[derive(Template)]
#[template(path = "post_deciduously-com.html")]
pub struct Blog3Template {
    links: &'static [Hyperlink],
}
impl Default for Blog3Template {
    fn default() -> Self {
        Self { links: &NAV }
    }
}

#[derive(Template)]
#[template(path = "post_green-squares.html")]
pub struct Blog0Template {
    links: &'static [Hyperlink],
}
impl Default for Blog0Template {
    fn default() -> Self {
        Self { links: &NAV }
    }
}

#[derive(Template)]
#[template(path = "post_actix-wrap.html")]
pub struct Blog1Template {
    links: &'static [Hyperlink],
}
impl Default for Blog1Template {
    fn default() -> Self {
        Self { links: &NAV }
    }
}

#[derive(Template)]
#[template(path = "post_cpp-template-specialization.html")]
pub struct Blog2Template {
    links: &'static [Hyperlink],
}
impl Default for Blog2Template {
    fn default() -> Self {
        Self { links: &NAV }
    }
}

#[derive(Template)]
#[template(path = "post_deciduously.html")]
pub struct Blog4Template {
    links: &'static [Hyperlink],
}
impl Default for Blog4Template {
    fn default() -> Self {
        Self { links: &NAV }
    }
}

#[derive(Template)]
#[template(path = "post_rust-arena-trees.html")]
pub struct Blog5Template {
    links: &'static [Hyperlink],
}
impl Default for Blog5Template {
    fn default() -> Self {
        Self { links: &NAV }
    }
}

#[derive(Template)]
#[template(path = "post_multi-stage-docker.html")]
pub struct Blog6Template {
    links: &'static [Hyperlink],
}
impl Default for Blog6Template {
    fn default() -> Self {
        Self { links: &NAV }
    }
}

pub async fn blog_handler(path_str: &str) -> HandlerResult {
    match path_str {
        "/green-squares" => {
            string_handler(
                &Blog0Template::default()
                    .render()
                    .expect("Should render markup"),
                "text/html",
                None,
            )
            .await
        }
        "/actix-wrap" => {
            string_handler(
                &Blog1Template::default()
                    .render()
                    .expect("Should render markup"),
                "text/html",
                None,
            )
            .await
        }
        "/cpp-template-specialization" => {
            string_handler(
                &Blog2Template::default()
                    .render()
                    .expect("Should render markup"),
                "text/html",
                None,
            )
            .await
        }
        "/deciduously" => {
            string_handler(
                &Blog4Template::default()
                    .render()
                    .expect("Should render markup"),
                "text/html",
                None,
            )
            .await
        }
        "/rust-arena-trees" => {
            string_handler(
                &Blog5Template::default()
                    .render()
                    .expect("Should render markup"),
                "text/html",
                None,
            )
            .await
        }
        "/multi-stage-docker" => {
            string_handler(
                &Blog6Template::default()
                    .render()
                    .expect("Should render markup"),
                "text/html",
                None,
            )
            .await
        }
        "/deciduously-com" => {
            string_handler(
                &Blog3Template::default()
                    .render()
                    .expect("Should render markup"),
                "text/html",
                None,
            )
            .await
        }
        _ => four_oh_four().await,
    }
}
