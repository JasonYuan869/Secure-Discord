use wasm_bindgen::JsError;
use libsignal_protocol::{AliceSignalProtocolParameters, BobSignalProtocolParameters, IdentityKey, IdentityKeyPair, initialize_alice_session_record, initialize_bob_session_record, KeyPair, PrivateKey, PublicKey, SessionRecord};
use rand::rngs::OsRng;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub struct ClientSession {
    session_record: SessionRecord,
}

#[wasm_bindgen]
impl ClientSession {
    pub fn new_alice(
        our_identity_key_pair_bytes: &[u8], // IKa
        their_identity_key_bytes: &[u8],    // IKb
        their_signed_pre_key_bytes: &[u8],  // SPKb
        their_one_time_pre_key_bytes: Option<Box<[u8]>> // OPKb
    ) -> Result<ClientSession, JsError> {
        let our_identity_key_pair = IdentityKeyPair::try_from(our_identity_key_pair_bytes)?;
        let our_base_key_pair = KeyPair::generate(&mut OsRng);
        let their_identity_key = IdentityKey::decode(their_identity_key_bytes)?;
        let their_signed_pre_key = PublicKey::deserialize(their_signed_pre_key_bytes)?;

        let mut params = AliceSignalProtocolParameters::new(
            our_identity_key_pair,
            our_base_key_pair,
            their_identity_key,
            their_signed_pre_key.clone(),
            their_signed_pre_key, // Use the signed pre key as ratchet initialization per documentation
        );

        if let Some(b) = their_one_time_pre_key_bytes {
            let their_one_time_pre_key = PublicKey::deserialize(b.as_ref())?;
            params.set_their_one_time_pre_key(their_one_time_pre_key);
        }

        let session_record = initialize_alice_session_record(&params, &mut OsRng)?;
        Ok(ClientSession { session_record })
    }

    pub fn new_bob(
        our_identity_key_pair_bytes: &[u8],
        our_signed_pre_key_private_bytes: &[u8],
        our_one_time_pre_key_private_bytes: Option<Box<[u8]>>,
        their_identity_key_bytes: &[u8],
        their_base_key_bytes: &[u8],
    ) -> Result<ClientSession, JsError> {
        let our_identity_key_pair = IdentityKeyPair::try_from(our_identity_key_pair_bytes)?;

        let our_signed_pre_key_pair_private = PrivateKey::deserialize(our_signed_pre_key_private_bytes)?;
        let our_signed_pre_key_pair = KeyPair::try_from(our_signed_pre_key_pair_private)?;

        let our_one_time_pre_key_pair = match our_one_time_pre_key_private_bytes {
            Some(b) => {
                let our_one_time_pre_key_private = PrivateKey::deserialize(b.as_ref())?;
                Some(KeyPair::try_from(our_one_time_pre_key_private)?)
            },
            None => None,
        };

        let their_identity_key = IdentityKey::decode(their_identity_key_bytes)?;
        let their_base_key = PublicKey::deserialize(their_base_key_bytes)?;

        let params = BobSignalProtocolParameters::new(
            our_identity_key_pair,
            our_signed_pre_key_pair,
            our_one_time_pre_key_pair.clone(),
            our_signed_pre_key_pair,
            None,
            their_identity_key,
            their_base_key,
            None,
        );

        let session_record = initialize_bob_session_record(&params)?;
        Ok(ClientSession { session_record })
    }

    pub fn serialize(&self) -> Box<[u8]> {
        self.session_record
            .serialize()
            .expect("could not serialize session record")
            .into_boxed_slice()
    }

    pub fn deserialize(value: &[u8]) -> Result<ClientSession, JsError> {
        let session_record = SessionRecord::deserialize(value)?;
        Ok(ClientSession { session_record })
    }
}
