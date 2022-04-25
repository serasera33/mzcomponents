use zoon::{route, static_ref, Router, *};
use app::*;
use crate::app;

#[route]
#[derive(Debug, Clone, Copy)]
pub enum Route {
    #[route()]
    Root,
}

#[static_ref]
pub fn router() -> &'static Router<Route> {
    Router::new(|route| match route {
        Some(Route::Root) => {
            zoon::println!("Root route requested");
            set_page_id(PageId::Root)
            },
        None => {}
        })
}
