use deciduously_com_content::{BlogPost, Content};
use deciduously_com_layouts::{document::Document, page_layout::PageLayout};
use deciduously_com_ui as ui;
use pinwheel::prelude::*;

#[derive(new)]
pub struct Page {
	slug: String,
}

impl Component for Page {
	fn into_node(self) -> Node {
		let blog_post = BlogPost::from_slug(self.slug).unwrap();
		let tags = if let Some(tags) = blog_post.front_matter.tags {
			let list_items = format!(
				"tags: {}",
				tags.iter()
					.map(|tag| format!("#{}", tag))
					.collect::<Vec<_>>()
					.join(", ")
			);
			div().class("tags").child(ol().child(list_items))
		} else {
			div()
		};
		let cover_image = if let Some(cover_image) = blog_post.front_matter.cover_image {
			div().class("cover-image").child(
				ui::Img::new()
					.alt("cover_image".to_owned())
					.src(cover_image.to_string()),
			)
		} else {
			div()
		};
		let heading = div()
			.style("line-height", "1.5")
			.child(h1().child(blog_post.front_matter.title.clone()))
			.child(
				div()
					.class("blog-post-date")
					.child(format!("{}", blog_post.front_matter.date)),
			)
			.child(cover_image)
			.child(tags);
		Document::new()
			.child(
				PageLayout::new().child(
					div()
						.class("blog-post-content")
						.child(div().class("s1").child(heading).child(blog_post.markdown)),
				),
			)
			.into_node()
	}
}
