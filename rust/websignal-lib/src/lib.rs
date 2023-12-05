mod utils;
mod keys;
mod session;

use wasm_bindgen::prelude::*;
use libsignal_protocol::IdentityKeyStore;

#[wasm_bindgen(start)]
pub fn init() {
    web_sys::console::log_1(&"Initializing wasm script".into());
    utils::set_panic_hook();
}


