use pinwheel::prelude::*;

pub struct Footer;

impl Component for Footer {
	fn into_node(self) -> Node {
		div()
			.class("footer-wrapper")
			.child(p().class("footer-copyright").child(format!(
				"Copyright © {} deciduously",
				time::OffsetDateTime::now_utc().year()
			)))
			.into_node()
	}
}
