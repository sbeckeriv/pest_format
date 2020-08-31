#![recursion_limit = "1024"]
extern crate console_error_panic_hook;
extern crate pest_fmt;
mod pages;
use std::panic;

use pages::Home;
//use std::panic;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[wasm_bindgen(start)]
pub fn run_app() {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    App::<Home>::new().mount_to_body();
}
