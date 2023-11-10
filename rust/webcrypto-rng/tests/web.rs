//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use wasm_bindgen_test::*;
use webcrypto_rng::*;
use rand_core::RngCore;
use web_sys::console;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn test_u32() {
    let window = web_sys::window().unwrap();
    let crypto = window.crypto().unwrap();
    let mut rng = WebRng::new(crypto);
    for i in 0..10 {
        console::log_2(&"generated value".into(), &rng.next_u32().into());
    }
}
