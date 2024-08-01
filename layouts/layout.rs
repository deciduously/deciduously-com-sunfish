use crate::footer::Footer;
use deciduously_com_sunfish_ui as ui;
use pinwheel::prelude::*;

#[derive(children, Default, new)]
#[new(default)]
pub struct Layout {
	pub children: Vec<Node>,
}

impl Component for Layout {
	fn into_node(self) -> Node {
		div()
			.class("layout")
			.child(header().child(Topbar))
			.child(main().child(self.children))
			.child(footer().child(Footer))
			.into_node()
	}
}

struct Topbar;

impl Component for Topbar {
	fn into_node(self) -> Node {
		let topbar_items = vec![
			ui::TopbarItem {
				element: None,
				href: "/".to_owned(),
				title: "Home".to_owned(),
			},
			ui::TopbarItem {
				element: None,
				href: "/blog/".to_owned(),
				title: "Blog".to_owned(),
			},
		];
		ui::Topbar::new()
			.background_color(ui::colors::HEADER.to_owned())
			.items(topbar_items)
			.title("deciduously.com".to_owned())
			.into_node()
	}
}
