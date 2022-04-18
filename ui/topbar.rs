use pinwheel::prelude::*;

#[derive(builder, Default, new)]
#[new(default)]
pub struct Topbar {
	#[builder]
	pub background_color: Option<String>,
	#[builder]
	pub items: Option<Vec<TopbarItem>>,
	#[builder]
	pub title: Option<String>,
}

pub struct TopbarItem {
	pub element: Option<Node>,
	pub href: String,
	pub title: String,
}

impl Component for Topbar {
	fn into_node(self) -> Node {
		let items = self.items.map(|items| {
			TopbarItemsWrapper::new().children(items.into_iter().map(|item| {
				if let Some(element) = item.element {
					element
				} else {
					a().class("topbar-link")
						.attribute("href", item.href)
						.child(item.title)
						.into_node()
				}
			}))
		});
		div().class("topbar-wrapper").child(items).into_node()
	}
}

#[derive(children, Default, new)]
#[new(default)]
struct TopbarItemsWrapper {
	pub children: Vec<Node>,
}

impl Component for TopbarItemsWrapper {
	fn into_node(self) -> Node {
		nav()
			.class("topbar-items-wrapper")
			.child(self.children)
			.into_node()
	}
}
