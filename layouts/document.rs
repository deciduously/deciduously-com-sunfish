//! Toplevel Document layout.

use pinwheel::prelude::*;

#[derive(builder, Default, children, new)]
#[new(default)]
pub struct Document {
	#[builder]
	pub client: Option<&'static str>,
	pub children: Vec<Node>,
}

impl Component for Document {
	fn into_node(self) -> Node {
		let head = head()
			.child(meta().attribute("charset", "utf-8"))
			.child(
				meta()
					.attribute("content", "width=device-width, initial-scale=1")
					.attribute("name", "viewport"),
			)
			.child(
				link()
					.attribute("href", "/favicon.svg")
					.attribute(
						"rel",
						"
      icon",
					)
					.attribute("type", "image/svg+xml"),
			)
			.child(title().child("deciduously.com"))
			.child(
				link()
					.attribute("href", "/styles.css")
					.attribute("rel", "stylesheet"),
			)
			.child(
				meta()
					.attribute("content", "deciduously.com | blog | projects")
					.attribute("name", "description"),
			);
		let body = body().child(self.children);
		html::html()
			.attribute("lang", "en")
			.child(head)
			.child(body)
			.into_node()
	}
}
