use libsignal_protocol::{IdentityKeyPair, PrivateKey, PublicKey};
use libsignal_protocol::KeyPair;
use rand::rngs::OsRng;
use wasm_bindgen::JsError;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct Identity {
    identity_key: IdentityKeyPair,
}

#[wasm_bindgen]
impl Identity {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Identity {
        Identity {
            identity_key: IdentityKeyPair::generate(&mut OsRng),
        }
    }

    pub fn serialize(&self) -> Box<[u8]> {
        self.identity_key.serialize()
    }

    pub fn deserialize(value: &[u8]) -> Result<Identity, JsError> {
        let identity_key = IdentityKeyPair::try_from(value)?;
        Ok(Identity { identity_key })
    }
}

#[wasm_bindgen]
/// A wrapper around a `KeyPair`.
pub struct Ephemeral {
    key: KeyPair,
}

#[wasm_bindgen]
impl Ephemeral {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Ephemeral {
        Ephemeral {
            key: KeyPair::generate(&mut OsRng),
        }
    }

    pub fn serialize_private(&self) -> Box<[u8]> {
        self.key.private_key.serialize().into_boxed_slice()
    }

    pub fn deserialize_private(value: &[u8]) -> Result<Ephemeral, JsError> {
        let private_key = PrivateKey::try_from(value)?;
        Ok(
            Ephemeral { key: KeyPair::try_from(private_key)? }
        )
    }
}

/// Wrapper around both `IdentityKey` and `PublicKey`.
#[wasm_bindgen]
pub struct Public {
    key: PublicKey,
}

#[wasm_bindgen]
impl Public {
    pub fn serialize(&self) -> Box<[u8]> {
        self.key.serialize()
    }

    pub fn deserialize(value: &[u8]) -> Result<Public, JsError> {
        let key = PublicKey::deserialize(value)?;
        Ok(
            Public {
                key
            }
        )
    }
}