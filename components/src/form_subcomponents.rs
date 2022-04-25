use zoon::*;
use crate::input::MultiLineInput;
//subcomponents to allow for Styles to be defined in one place

pub fn form_text(text: &str) -> impl Element {
    Paragraph::new()
        .content(text)
        .s(Background::new().color(named_color::YELLOW_0))
}

pub fn form_input<'a>(
    placeholder: &str,
    on_change: impl FnOnce(String) + Clone + 'static,
    text_signal: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
) -> impl Element {
    TextInput::new()
        .placeholder(Placeholder::new(placeholder))
        .label_hidden("")
        // .text_signal(on_change)
        .on_change(on_change)
        .text_signal(text_signal)
}


pub fn form_multiline_input<'a>(
    placeholder: &str,
    on_change: impl FnOnce(String) + Clone + 'static,
    text_signal: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
) -> impl Element {
    MultiLineInput::MultiLineInput::new()
        .placeholder(MultiLineInput::Placeholder::new(placeholder))
        .label_hidden("")
        .on_change(on_change)
        .text_signal(text_signal)
}

pub fn form_row<'a>(
    text: &str,
    placeholder: &str,
    on_change: impl FnOnce(String) + Clone + 'static,
    text_signal: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
) -> impl Element {
    Column::new()
        .item(form_text(text))
        .item(form_input(placeholder, on_change, text_signal))
}

pub fn form_md_input<'a>(
    placeholder: &str,
    on_change: impl FnOnce(String) + Clone + 'static,
    text_signal: impl Signal<Item = impl IntoOptionCowStr<'a>> + Unpin + 'static,
) -> impl Element {
    TextInput::new()
        .placeholder(Placeholder::new(placeholder))
        .label_hidden("")
        .on_change(on_change)
        .text_signal(text_signal)
}

pub fn form_md_row<'a>(
    text: &str,
    placeholder: &str,
    on_change: impl FnOnce(String) + Clone + 'static,
    content: impl Signal<Item = impl IntoCowStr<'a> + std::clone::Clone>  + Unpin + 'static,
    text_signal: impl Signal<Item = impl IntoCowStr<'a>> + Unpin + 'static,
) -> impl Element {
    Column::new().item(form_text(text)).item(
        Row::new()
            .item(form_multiline_input(placeholder, on_change, text_signal))
            .item(RawHtmlEl::new("div").inner_markup_signal(content).into_raw_element()),
    )
}


pub fn button_simple(text: &str, on_click_cl: impl FnOnce() + Clone + 'static) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let hovered_signal2 = hovered.signal();
    Button::new()
        .s(Borders::all_signal(hovered_signal.map_bool(
            || Border::new().color(hsluv!(50, 0, 0, 32)),
            || Border::new().color(hsluv!(0, 0, 0, 0)),
        )))
        .s(Background::new().color_signal({
            hovered_signal2.map_bool(|| named_color::BLUE_0, || named_color::YELLOW_0)
        }))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .label(text)
        .on_click(on_click_cl)
}

