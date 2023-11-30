mod utils;

use libsignal_protocol::{
    IdentityKeyPair,
    PrivateKey,
    PublicKey,
    error::Result,
};

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

impl Identity {
    pub fn new() -> Self {
        Identity {
            identity_key: IdentityKeyPair::generate(&mut OsRng)
        }
    }

    pub fn serialize(&self) -> Box<[u8]> {
        self.identity_key.serialize()
    }

    pub fn deserialize(value: &[u8]) -> Result<Self> {
        let identity_key = IdentityKeyPair::try_from(value)?;
        Ok(Identity { identity_key })
    }
}


