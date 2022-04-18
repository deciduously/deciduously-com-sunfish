use deciduously_com_content::{BlogPost, Content};
use deciduously_com_layouts::{document::Document, page_layout::PageLayout};
use pinwheel::prelude::*;

#[derive(new)]
pub struct Page {
	slug: String,
}

impl Component for Page {
	fn into_node(self) -> Node {
		let blog_post = BlogPost::from_slug(self.slug).unwrap();
		let author = if let Some(author) = blog_post.front_matter.author {
			Author::new().name(author.name)
		} else {
			Author::new().name("The Team")
		};
		let heading = div()
			.style("line-height", "1.5")
			.child(h1().child(blog_post.front_matter.title.clone()))
			.child(div().class("blog-post-date").child(format!(
				"Originally published on {}",
				blog_post.front_matter.date
			)))
			.child(author);
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

#[derive(builder, Default, new)]
#[new(default)]
pub struct Author {
	#[builder]
	pub name: String,
}

impl Component for Author {
	fn into_node(self) -> Node {
		div()
			.class("blog-post-author")
			.child(div().child(format!("By {}", self.name)))
			.into_node()
	}
}
