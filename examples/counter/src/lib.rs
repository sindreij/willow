#![feature(arbitrary_self_types)]

mod app;
#[macro_use]
mod utils;

// use std::mem;
use std::rc::Rc;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
pub fn main() {
    utils::set_panic_hook();
    let program = Rc::new(app::main());
    program.start();
    // mem::forget(program);

    // let model = app::init();

    // let tree = app::view(&model);

    // console_log!("View: {:#?}", tree);

    // if let Err(err) = render::render(&tree) {
    //     console_log!("Got error: {:?}", err);
    // }
}
