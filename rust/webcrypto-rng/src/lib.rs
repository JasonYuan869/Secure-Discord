#![no_std]

mod utils;

use utils::set_panic_hook;
use rand_core::{RngCore, CryptoRng, Error};
use web_sys::{Crypto};

pub struct WebRng(Crypto);

impl WebRng {
    pub fn new(crypto: Crypto) -> Self {
        set_panic_hook();
        Self(crypto)
    }
}

impl RngCore for WebRng {
    fn next_u32(&mut self) -> u32 {
        let mut buffer: [u8; 4] = [0; 4];
        self.0.get_random_values_with_u8_array(&mut buffer).expect("Error getting random values");

        u32::from_ne_bytes(buffer)
    }

    fn next_u64(&mut self) -> u64 {
        let mut buffer: [u8; 8] = [0; 8];
        self.0.get_random_values_with_u8_array(&mut buffer).expect("Error getting random values");

        u64::from_ne_bytes(buffer)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        self.0.get_random_values_with_u8_array(dest).expect("Error getting random values");
    }

    fn try_fill_bytes(&mut self, dest: &mut [u8]) -> Result<(), Error> {
        match self.0.get_random_values_with_u8_array(dest) {
            Ok(_) => Ok(()),
            Err(_) => {
                let code = core::num::NonZeroU32::new(Error::CUSTOM_START + 1).unwrap();
                Err(Error::from(code))
            }
        }

    }
}

impl CryptoRng for WebRng {}
