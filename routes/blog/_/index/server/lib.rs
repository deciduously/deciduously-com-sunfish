use deciduously_com_sunfish_content::{BlogPost, Content};
use pinwheel::prelude::*;

mod page;

#[must_use]
pub fn init() -> sunfish::Route {
	sunfish::Route::new_static_with_paths(
		|| {
			BlogPost::slugs()
				.unwrap()
				.into_iter()
				.map(|slug| format!("/blog/{slug}/"))
				.collect()
		},
		|path| {
			let ["blog", slug, ""] = *sunfish::path_components(&path).as_slice() else {
				panic!()
			};
			html(self::page::Page::new(slug.to_owned()))
		},
	)
}
