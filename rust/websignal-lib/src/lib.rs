mod utils;

use libsignal_protocol::kem::{KeyPair, KeyType};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
}
