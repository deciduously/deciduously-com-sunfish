use deciduously_com_layouts::{document::Document, page_layout::PageLayout};
use deciduously_com_ui as ui;
use pinwheel::prelude::*;

pub struct Page;

impl Component for Page {
	fn into_node(self) -> Node {
		Document::new()
			.child(
				PageLayout::new().child(
					div()
						.class("index-content")
						.child(h1().class("index-title").child("deciduously"))
						.child(div().class("s1").child("Ben Lovy | Rust Developer"))
						.child(
							ul().child(
								li().child(
									ui::Card::new().child("DEV.to: ").child(
										ui::Link::new()
											.href("https://dev.to/deciduously".to_owned())
											.target("blank".to_owned())
											.child("deciduously".to_owned()),
									),
								),
							)
							.child(
								li().child(
									ui::Card::new().child("Github: ").child(
										ui::Link::new()
											.href("https://github.com/deciduously".to_owned())
											.target("blank".to_owned())
											.child("deciduously".to_owned()),
									),
								),
							),
						),
				),
			)
			.into_node()
	}
}
