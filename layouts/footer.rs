use deciduously_com_ui as ui;
use pinwheel::prelude::*;

pub struct Footer;

impl Component for Footer {
	fn into_node(self) -> Node {
		div()
			.class("footer-wrapper")
			.child(
				p().class("footer-copyright")
					.child(format!(
						"Copyright © {} deciduously | ",
						time::OffsetDateTime::now_utc().year()
					))
					.child(
						ui::Link::new()
							.href("https://github.com/deciduously/deciduously-com".to_owned())
							.target("blank".to_owned())
							.child("source".to_owned()),
					),
			)
			.into_node()
	}
}
