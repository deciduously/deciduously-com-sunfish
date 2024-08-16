use pinwheel::prelude::*;

mod page;

#[must_use]
pub fn init() -> sunfish::Route {
	sunfish::Route::new_static(|_| html(self::page::Page))
}
