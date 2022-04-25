use zoon::*;
use zoon::signal::MutableLockMut;
use components::form_subcomponents::{button_simple, form_row};
use components::selection::selector::Selector;
use components::view_subcomponents::view_text;
use shared::Favorites;

#[static_ref]
pub fn current_fav() -> &'static Mutable<Favorites> {
    Mutable::new(Favorites::init())
}

#[static_ref]
pub fn all_favs() -> &'static MutableVec<Favorites> {
    MutableVec::new_with_values(vec![Favorites {
        name: "Charlie".to_string(),
        colors: vec!["Red".to_string(), "Blue".to_string()],
        dishes: vec!["Spaghetti".to_string(), "Pizza".to_string()],
    },
                                     Favorites {
                                         name: "Chaplin".to_string(),
                                         colors: vec!["Green".to_string()],
                                         dishes: vec!["Spaghetti".to_string(), "Pizza".to_string()],
                                     }])
}

// #[static_ref]
// fn COLOR_CHOICES() -> &'static Vec<String> {
//     vec!(String::from("red"), String::from("green"), String::from("blue"))
// }

fn fav_exist() -> impl Signal<Item=bool> {
    fav_count().map(|count| count != 0).dedupe()
}

fn fav_count() -> impl Signal<Item=usize> {
    all_favs().signal_vec_cloned().len()
}

pub fn page() -> impl Element {
    Row::new()
        .item(fav_list())
        .item(fav_entry())
}

fn fav_list() -> impl Element {
    Column::new()
        .s(Width::new(400))
        .s(zoon::Align::new().top())
        .item(view_text("Favorites (Click to Select)"))
        .items_signal_vec(all_favs().signal_vec_cloned().map(fav_item))
}

fn fav_item(fav: Favorites) -> impl Element {
    let (hovered, hovered_signal) = Mutable::new_and_signal(false);
    Row::new()
        .s(Borders::all_signal(hovered_signal.map_bool(
            || Border::new().color(named_color::YELLOW_2),
            || Border::new().color(hsluv!(0, 0, 0, 0)),
        )))
        .s(Width::new(400))
        .on_hovered_change(move |is_hovered| hovered.set_neq(is_hovered))
        .item(view_text(fav.name.as_str()))
        .item(view_text(format!("{:?}", fav).as_str()))
        .on_click(|| {
            zoon::println!("Selected {:?}", fav);
            current_fav().replace(fav);
            let lock = current_fav().lock_mut();
            lock.colors.iter().for_each(|x| selected_colors().lock_mut().push_cloned((*x).clone()));
            lock.dishes.iter().for_each(|x| selected_dishes().lock_mut().push_cloned((*x).clone()));
        })
}

#[static_ref]
fn color_options() -> &'static MutableVec<String> {
    MutableVec::new_with_values(vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()])
}

#[static_ref]
pub fn color_selector() -> &'static Selector {
    Selector::new((*color_options().lock_mut()).to_vec())
}

#[static_ref]
pub fn selected_colors() -> &'static MutableVec<String> {
    MutableVec::new()
}

#[static_ref]
fn dish_options() -> &'static MutableVec<String> {
    MutableVec::new_with_values(vec!["Spaghetti".to_string(), "Pizza".to_string(), "Risotto".to_string()])
}

#[static_ref]
pub fn dish_selector() -> &'static Selector {
    Selector::new((*dish_options().lock_mut()).to_vec())
}

#[static_ref]
pub fn selected_dishes() -> &'static MutableVec<String> {
    MutableVec::new()
}


fn fav_entry() -> impl Element {

    Column::new()
        .s(Width::new(400))
        .s(zoon::Align::new().top())
        .s(Padding::new().left(25))
        .item(form_row(
            "Name (required)",
            "Enter Task Name",
            |x| {
                let mut lock: MutableLockMut<Favorites> = current_fav().lock_mut();
                lock.name = x.parse::<String>().unwrap_or_else(|_| String::from(""));
            },
            current_fav().signal_ref(|x| {
                let txt: String = x.name.clone();
                txt
            }),
        ))
        .item(color_selector()
            .checked_items_signal(selected_colors().signal_vec_cloned())
            .to_component("Favorite Colors"))
        .item(dish_selector()
            .checked_items_signal(selected_dishes().signal_vec_cloned())
            .to_component("Favorite Dishes"))
        .item(button_simple("Clear",|| {
            current_fav().replace( Favorites::init());
        }))
}