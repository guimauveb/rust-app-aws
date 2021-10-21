#![recursion_limit = "512"]

#[macro_use]
extern crate dotenv_codegen;

use wasm_bindgen::prelude::*;

pub mod app;
pub mod components;
pub mod entities;
pub mod routes;
pub mod service;
pub mod utils;

const API_URL: &str = dotenv!("API_URL");

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn run_app() -> Result<(), JsValue> {
    #[cfg(debug_assertions)]
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<app::App>();
    Ok(())
}

fn main() {
    if let Err(err) = run_app() {
        println!("{:#?}", err)
    };
}
