use core::num::NonZeroU32;
use rand_core::Error;

pub const TRY_FILL_BYTES_ERROR: NonZeroU32 = unsafe { NonZeroU32::new_unchecked(Error::CUSTOM_START + 1) };
