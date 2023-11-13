#![no_std]

mod utils;
mod errors;

use utils::set_panic_hook;
use rand_core::{RngCore, CryptoRng, Error, impls};
use web_sys::Crypto;
use crate::errors::TRY_FILL_BYTES_ERROR;

pub struct WebRng(Crypto);

impl WebRng {
    pub fn new(crypto: Crypto) -> Self {
        set_panic_hook();
        Self(crypto)
    }
}

impl RngCore for WebRng {
    fn next_u32(&mut self) -> u32 {
        impls::next_u32_via_fill(self)
    }

    fn next_u64(&mut self) -> u64 {
        impls::next_u64_via_u32(self)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.get_random_values_with_u8_array(dest).expect("Error getting random values");
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        match self.0.get_random_values_with_u8_array(dest) {
            Ok(_) => Ok(()),
            Err(_) => {
                Err(Error::from(TRY_FILL_BYTES_ERROR))
            }
        }

    }
}

impl CryptoRng for WebRng {}
