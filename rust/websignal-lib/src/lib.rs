mod utils;

use libsignal_protocol::{IdentityKeyPair};
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub struct Identity {
    identity_key: IdentityKeyPair,
}

#[wasm_bindgen]
impl Identity {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Identity {
            identity_key: IdentityKeyPair::generate(&mut OsRng),
        }
    }

    pub fn serialize(&self) -> Box<[u8]> {
        self.identity_key.serialize()
    }

    pub fn deserialize(value: &[u8]) -> Result<Identity, JsError> {
        if let Ok(identity_key) = IdentityKeyPair::try_from(value) {
            Ok(Identity { identity_key: identity_key })
        } else {
            Err(JsError::new("Failed to deserialize identity key"))
        }
    }
}


