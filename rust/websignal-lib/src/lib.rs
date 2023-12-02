mod utils;
// mod stores;

use libsignal_protocol::{IdentityKey, IdentityKeyPair, ProtocolAddress, SessionRecord, initialize_alice_session_record, initialize_bob_session_record, AliceSignalProtocolParameters, BobSignalProtocolParameters};
use rand::rngs::OsRng;
use wasm_bindgen::prelude::*;
// use crate::stores::JsProtocolStore;
use libsignal_protocol::IdentityKeyStore;

#[wasm_bindgen(start)]
pub fn init() {
    web_sys::console::log_1(&"Initializing wasm script".into());
    utils::set_panic_hook();
}

// #[wasm_bindgen]
// pub async fn test() {
//     let self_identity_key = IdentityKeyPair::generate(&mut OsRng);
//     let mut store = JsProtocolStore::new(self_identity_key, 1);
//     let new_identity_key = IdentityKeyPair::generate(&mut OsRng);
//     let public_key = new_identity_key.public_key();
//
//     store.save_identity(&ProtocolAddress::new("bob".to_string(), 0.into()), &IdentityKey::from(*public_key)).await.expect("Error saving identity");
// }

pub struct JsAliceParams {

}

#[wasm_bindgen]
pub struct ClientSession {
    session_record: SessionRecord,
}

#[wasm_bindgen]
impl ClientSession {
    #[wasm_bindgen(constructor)]
    pub fn new_alice() -> Result<Self, JsError> {
        let session_record = initialize_alice_session_record(, &mut OsRng)?;
        Ok(ClientSession { session_record })
    }

    #[wasm_bindgen(constructor)]
    pub fn new_bob() -> Result<Self, JsError> {
        let session_record = initialize_bob_session_record(, &mut OsRng)?;
        Ok(ClientSession { session_record })
    }

    pub fn serialize(&self) -> Box<[u8]> {
        self.session_record
            .serialize()
            .expect("could not serialze session record")
            .into_boxed_slice()
    }

    pub fn deserialize(value: &[u8]) -> Result<ClientSession, JsError> {
        let session_record = SessionRecord::deserialize(value)?;
        Ok(ClientSession { session_record })
    }
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
        let identity_key = IdentityKeyPair::try_from(value)?;
        Ok(Identity { identity_key })
    }
}
