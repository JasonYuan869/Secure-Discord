use async_trait::async_trait;
use libsignal_protocol::{IdentityKey, ProtocolAddress, SignalProtocolError, error::Result, Direction, ProtocolStore, IdentityKeyStore, IdentityKeyPair};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::js_sys::{Promise, Uint8Array};
use wasm_bindgen_futures::JsFuture;
#[wasm_bindgen(module = "/ProtocolStore.js")]
extern "C" {
    type SignalProtocolStore;

    #[wasm_bindgen(constructor)]
    fn new(key_pair: &[u8], registration_id: u32) -> SignalProtocolStore;

    #[wasm_bindgen(method, js_name = "getIdentityKeyPair")]
    fn get_identity_key_pair(this: &SignalProtocolStore) -> Promise; //JsResult<&[u8]>;

    #[wasm_bindgen(method, js_name = "getLocalRegistrationId")]
    fn get_local_registration_id(this: &SignalProtocolStore) -> Promise; //JsResult<u32>;

    #[wasm_bindgen(method, js_name = "saveIdentity")]
    fn save_identity(this: &SignalProtocolStore, username: &str, identity_key: &[u8]) -> Promise; //JsResult<bool>;

    #[wasm_bindgen(method, js_name = "isTrustedIdentity")]
    fn is_trusted_identity(this: &SignalProtocolStore, username: &str, identity_key: &[u8], direction: bool) -> Promise; //JsResult<bool>;

    #[wasm_bindgen(method, js_name = "getIdentity")]
    fn get_identity(this: &SignalProtocolStore, username: &str) -> Promise; //JsResult<Option<&[u8]>>;
}

pub(crate) struct JsProtocolStore {
    inner: SignalProtocolStore,
}

impl JsProtocolStore {
    pub fn new(key_pair: IdentityKeyPair, registration_id: u32) -> Self {
        JsProtocolStore {
            inner: SignalProtocolStore::new(key_pair.serialize().as_ref(), registration_id),
        }
    }
}

#[async_trait(?Send)]
impl IdentityKeyStore for JsProtocolStore {
    async fn get_identity_key_pair(&self) -> Result<IdentityKeyPair> {
        if let Ok(key) = JsFuture::from(self.inner.get_identity_key_pair()).await {
            Ok(IdentityKeyPair::try_from(Uint8Array::from(key).to_vec().as_slice()).unwrap())
        } else {
            Err(SignalProtocolError::InvalidArgument("Failed to get identity key pair".to_string()))
        }
    }

    async fn get_local_registration_id(&self) -> Result<u32> {
        if let Ok(registration_id) = JsFuture::from(self.inner.get_local_registration_id()).await {
            Ok(registration_id.as_f64().unwrap() as u32)
        } else {
            Err(SignalProtocolError::InvalidArgument("Failed to get local registration id".to_string()))
        }
    }

    async fn save_identity(&mut self, address: &ProtocolAddress, identity_key: &IdentityKey) -> Result<bool> {
        if let Ok(saved) = JsFuture::from(self.inner.save_identity(address.name(), identity_key.serialize().as_ref())).await {
            Ok(saved.as_bool().unwrap())
        } else {
            Err(SignalProtocolError::InvalidArgument("Failed to save identity".to_string()))
        }
    }

    async fn is_trusted_identity(&self, address: &ProtocolAddress, identity_key: &IdentityKey, direction: Direction) -> Result<bool> {
        let direction_bool = direction == Direction::Sending;
        if let Ok(trusted) = JsFuture::from(self.inner.is_trusted_identity(address.name(), identity_key.serialize().as_ref(), direction_bool)).await {
            Ok(trusted.as_bool().unwrap())
        } else {
            Err(SignalProtocolError::InvalidArgument("Failed to check if identity is trusted".to_string()))
        }
    }

    async fn get_identity(&self, address: &ProtocolAddress) -> Result<Option<IdentityKey>> {
        if let Ok(identity) = JsFuture::from(self.inner.get_identity(address.name())).await {
            if identity.is_truthy() {
                Ok(Some(IdentityKey::try_from(Uint8Array::from(identity).to_vec().as_slice()).unwrap()))
            } else {
                Ok(None)
            }
        } else {
            Err(SignalProtocolError::InvalidArgument("Failed to get identity".to_string()))
        }
    }
}
