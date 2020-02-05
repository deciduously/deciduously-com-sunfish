use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, Default)]
pub struct LinkInfo {
    pub id: usize,
    pub url_name: &'static str,
    pub title: &'static str,
}

#[derive(Debug, Default)]
pub struct BlogLinkInfo {
    pub drafts: Vec<LinkInfo>,
    pub published: Vec<LinkInfo>,
}

lazy_static! {
    pub static ref LINKINFO: BlogLinkInfo = {
        let mut ret = BlogLinkInfo::default();
        ret.drafts.push(LinkInfo { id: 0, title: "I Scrapped My Stencil Project And WrA", url_name: "/deciduously-com-unpublished" });
        ret.published.push(LinkInfo { id: 1, title: "I Scrapped My Stencil Project And Wrote A Static Site Instead", url_name: "/deciduously-com" });
        ret
    };
}
