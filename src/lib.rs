mod gamestate;
mod utils;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
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
