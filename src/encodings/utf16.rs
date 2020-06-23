// Generate invalid byte given a RNG
// Decode UTF-16-encoded BadSV:
// - Parse until invalid UTF-16
// - Split at delimiters, return vec of vec
// Encode UTF-16-flavored BadSV:
// - Get random invalid bytes
// - Write binary file, values joined with invalid delimiters

use rand::Rng;
use rand::thread_rng;
use crate::encodings::{Encoder, Decoder, DecodeResult, Encoding};

pub const MIN_BYTES: usize = 2;

// Surrogate halves that can't start a surrogate pair
const INVALID_UTF16_HALF_START: u16 = 0xd800;
const INVALID_UTF16_HALF_END: u16 = 0xdfff;

pub struct Utf16Encoder {
    size: usize
}
pub struct Utf16Decoder {
    size: usize
}

impl Utf16Encoder {
    pub fn new() -> Self { Utf16Encoder { size: MIN_BYTES } }
}

impl Utf16Decoder {
    pub fn new() -> Self { Utf16Decoder { size: MIN_BYTES } }
}

impl Encoder for Utf16Encoder {
    /// Encodes a utf-16 string
    fn encode(&self, data: &str) -> Vec<u8> {
        let mut out = Vec::new();
        for pair in data.encode_utf16() {
            out.push((pair >> 8) as u8);
            out.push((pair & 255) as u8);
        }
        out
    }

    /// Generates a random invalid utf-16 surrogate half
    fn get_delimiter(&self) -> Vec<u8> {
        let bad_surrogate = thread_rng().gen_range(INVALID_UTF16_HALF_START, INVALID_UTF16_HALF_END);
        vec![(bad_surrogate >> 8) as u8, (bad_surrogate & 255) as u8]
    }
}

impl Decoder for Utf16Decoder {
    /// Decode bytes from a utf-16 String as far as possible
    fn try_decode(&self, bytes: &[u8]) -> DecodeResult {
        // Join each pair of bytes
        let merged: Vec<u16> = bytes
            .chunks_exact(2)
            .map(|chunk| (chunk[0] as u16) << 8 | chunk[1] as u16) 
            .collect();
        let mut original_repl = Vec::new();
        for (i, &pair) in merged.iter().enumerate() {
            if pair == 0xfffd {
                original_repl.push(i);
            }
        }
        let out = String::from_utf16_lossy(&merged);
        for (i, c) in out.chars().enumerate() {
            if c == '\u{FFFD}' && !original_repl.contains(&i) {
                return Err((String::from(&out[..i]), i))
            }
        }
        Ok(out)
    }
}


impl Encoding for Utf16Encoder {
    fn size(&self) -> usize {
        self.size
    }
}

impl Encoding for Utf16Decoder {
    fn size(&self) -> usize {
        self.size
    }
}
