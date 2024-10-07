use wasm_bindgen::prelude::*;
// web-sys
// gloo
// serde

// components
mod component;
mod lua_logic;
use component::index::IndexComponent;


#[wasm_bindgen(start)]
fn run_app() {
    yew::Renderer::<IndexComponent>::new().render();
}