use deciduously_com_content::{BlogPost, Content};
use deciduously_com_layouts::{document::Document, page_layout::PageLayout};
use deciduously_com_ui as ui;
use pinwheel::prelude::*;

pub struct Page;

impl Component for Page {
	fn into_node(self) -> Node {
		let blog_posts = BlogPost::list().unwrap().into_iter().map(|blog_post| {
			let href = format!("/blog/{}/", blog_post.slug);
			div()
				.child(
					ui::Link::new()
						.href(href)
						.child(blog_post.front_matter.title),
				)
				.child(p().child(blog_post.front_matter.date))
		});
		Document::new()
			.child(
				PageLayout::new()
					.child(h1().child("Blog"))
					.child(div().class("s2").children(blog_posts)),
			)
			.into_node()
	}
}
