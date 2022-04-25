mod app;
mod header;
mod router;
mod pages;

use zoon::*;

#[wasm_bindgen(start)]
pub fn start() {
    zoon::println!("Starting APP");
    router::router();
    start_app("app", app::root);
}