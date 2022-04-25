use zoon::*;
use zoon::FontFamily::SansSerif;

use crate::header;
use crate::pages::selector_page;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum PageId {
    Root
}

#[static_ref]
pub fn page_id() -> &'static Mutable<PageId> {
    Mutable::new(PageId::Root)
}



pub fn set_page_id(new_page_id: PageId) {
    page_id().set_neq(new_page_id);
}

pub fn root() -> impl Element {
    Column::new()
        .s(Width::fill())
        .s(Font::new()
            .size(14)
            .color(hsluv!(0, 0, 5.1))
            .weight(FontWeight::Light)
            .family(vec![
                FontFamily::new("Helvetica Neue"),
                FontFamily::new("Helvetica"),
                FontFamily::new("Arial"),
                FontFamily::SansSerif,
            ]))
        .item(header::header())
        .item(page())
}

fn page() -> impl Element {
    RawHtmlEl::new("div").class("nav").child_signal(page_id().signal().map(|page_id| match page_id {
        PageId::Root => { selector_page::page().into_raw_element() }
    }))
}