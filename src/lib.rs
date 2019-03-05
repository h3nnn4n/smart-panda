extern crate rand;
extern crate wasm_bindgen;
extern crate cfg_if;

mod gamestate;
mod utils;

use wasm_bindgen::prelude::*;
use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn console_log(text: &str) {
    log!("{}", text);
}

#[wasm_bindgen]
pub fn js_alert(name: &str) {
    alert(&format!("{}", name));
}

#[wasm_bindgen]
pub fn get_new_gamestate() -> gamestate::GameState {
    utils::set_panic_hook();
    gamestate::GameState::new()
}
