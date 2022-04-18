use deciduously_com_layouts::{document::Document, page_layout::PageLayout};
use pinwheel::prelude::*;

pub struct Page;

impl Component for Page {
	fn into_node(self) -> Node {
		Document::new()
			.child(
				PageLayout::new().child(
					div()
						.class("index-content")
						.child(h1().class("index-title").child("Sunfish/Pinwheel Template")),
				),
			)
			.into_node()
	}
}
