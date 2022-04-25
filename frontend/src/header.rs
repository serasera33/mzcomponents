use zoon::*;
use crate::router::Route;

pub fn header() -> impl Element {
    Row::new()
        .item(button_nav("Selectors", Route::Root))
}

pub fn button_nav(lbl: &str, route: Route) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    let hovered_signal2 = hovered.signal();
    Link::new()
        .label(lbl)
        .s(Borders::all_signal(hovered_signal.map_bool(
            || Border::new().color(named_color::YELLOW_2),
            || Border::new().color(hsluv!(0, 0, 0, 0)),
        )))
        .s(Background::new().color_signal({
            hovered_signal2.map_bool(|| named_color::BLUE_4, || named_color::BLUE_2)
        }))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .to(route)
}