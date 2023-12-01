mod utils;
mod stores;

use libsignal_protocol::{IdentityKey, IdentityKeyPair, ProtocolAddress, SessionRecord};
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;
use crate::stores::JsProtocolStore;
use libsignal_protocol::IdentityKeyStore;

#[wasm_bindgen(start)]
pub fn init() {
    web_sys::console::log_1(&"Initializing wasm script".into());
    utils::set_panic_hook();
}

#[wasm_bindgen]
pub async fn test() {
    let self_identity_key = IdentityKeyPair::generate(&mut OsRng);
    let mut store = JsProtocolStore::new(self_identity_key, 1);
    let new_identity_key = IdentityKeyPair::generate(&mut OsRng);
    let public_key = new_identity_key.public_key();

    store.save_identity(&ProtocolAddress::new("bob".to_string(), 0.into()), &IdentityKey::from(*public_key)).await.expect("Error saving identity");
}

#[wasm_bindgen]
pub async fn test2() {

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
