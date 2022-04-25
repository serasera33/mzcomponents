use zoon::{*};
use crate::util::markdown_to_html;

pub fn view_text(text: &str) -> impl Element {
    Paragraph::new()
        .content(text)
        .s(Background::new().color(named_color::GRAY_0))
        .s(zoon::Align::new().center_x().center_y())
        .s(Padding::new().left(20))
}

pub fn view_md(text: &str) -> impl Element {
    RawHtmlEl::new("div")
        .inner_markup(markdown_to_html(String::from(text)))
        .into_raw_element()
        // .s(Background::new().color(named_color::GRAY_1))
}