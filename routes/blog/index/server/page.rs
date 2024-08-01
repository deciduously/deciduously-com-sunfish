use deciduously_com_sunfish_content::{BlogPost, Content};
use deciduously_com_sunfish_layouts::{document::Document, page_layout::PageLayout};
use deciduously_com_sunfish_ui as ui;
use pinwheel::prelude::*;
use time::format_description::FormatItem;

pub struct Page;

const DATE_FORMAT: &[FormatItem<'static>] =
	time::macros::format_description!("[month repr:long] [day padding:none], [year]");

impl Component for Page {
	fn into_node(self) -> Node {
		let blog_posts = BlogPost::list().unwrap().into_iter().map(|blog_post| {
			let date = blog_post.front_matter.date.format(&DATE_FORMAT).unwrap();
			let href = format!("/blog/{}/", blog_post.slug);
			div()
				.child(
					ui::Link::new()
						.href(href)
						.child(blog_post.front_matter.title),
				)
				.child(p().child(date))
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
