// Generate invalid byte given a RNG
// Decode UTF-16-encoded BadSV:
// - Parse until invalid UTF-16
// - Split at delimiters, return vec of vec
// Encode UTF-16-flavored BadSV:
// - Get random invalid bytes
// - Write binary file, values joined with invalid delimiters

use rand::Rng;
use rand::thread_rng;

use rand;

pub const MIN_BYTES: usize = 2;

// Surrogate halves that can't start a surrogate pair
const INVALID_UTF16_LE_START: u16 = 0xd800;
const INVALID_UTF16_LE_END: u16 = 0xdbff;

/// Generates a random invalid utf-16 surrogate half
pub fn get_delimiter() -> Vec<u8> {
    let bad_surrogate = thread_rng().gen_range(INVALID_UTF16_LE_START, INVALID_UTF16_LE_END);
    vec![(bad_surrogate >> 8) as u8, (bad_surrogate & 255) as u8]
}