// Generate invalid byte given a RNG
// Decode UTF-32-encoded BadSV:
// - Parse until invalid UTF-32
// - Split at delimiters, return vec of vec
// Encode UTF-32-flavored BadSV:
// - Get random invalid bytes
// - Write binary file, values joined with invalid delimiters

use std::iter::FromIterator;

use rand::Rng;
use rand::thread_rng;
use crate::encodings::{Encoder, Decoder, DecodeResult, Encoding};

pub const MIN_BYTES: usize = 4;

// codepoints only go up to 0x0010ffff
const INVALID_UTF32_HALF_START_MINUS_ONE: u32 = 0x0010ffff;
const INVALID_UTF32_HALF_END_MINUS_ONE: u32 = 0xffffffff;

pub struct Utf32Encoder {
    size: usize
}
pub struct Utf32Decoder {
    size: usize
}

impl Utf32Encoder {
    pub fn new() -> Self { Utf32Encoder { size: MIN_BYTES } }
}

impl Utf32Decoder {
    pub fn new() -> Self { Utf32Decoder { size: MIN_BYTES } }
}

impl Encoder for Utf32Encoder {
    /// Encodes a utf-32 string
    fn encode(&self, data: &str) -> Vec<u8> {
        let mut out = Vec::new();
        for c in data.chars() {
            let utf32 = c as u32;
            out.push((utf32 >> 24) as u8);
            out.push((utf32 >> 16) as u8);
            out.push((utf32 >> 8) as u8);
            out.push(utf32 as u8);
        }
        out
    }

    /// Generates a random invalid utf-32 byte sequence
    fn get_delimiter(&self) -> Vec<u8> {
        let bad = 1 + thread_rng().gen_range(INVALID_UTF32_HALF_START_MINUS_ONE, INVALID_UTF32_HALF_END_MINUS_ONE);
        vec![(bad >> 24) as u8, (bad >> 16) as u8, (bad >> 8) as u8, bad as u8]
    }
}

impl Decoder for Utf32Decoder {
    /// Decode bytes from a utf-32 String as far as possible
    fn try_decode(&self, bytes: &[u8]) -> DecodeResult {
        // Join each pair of bytes
        let mut error = None;
        let merged: Vec<_> = bytes
            .chunks_exact(4)
            .map(|chunk| (chunk[0] as u32) << 24 | (chunk[1] as u32) << 16 | (chunk[2] as u32) << 8 | chunk[3] as u32) 
            .enumerate()
            .inspect(|&(i, int)| {
                if int > INVALID_UTF32_HALF_START_MINUS_ONE {
                    error = Some(i)
                }
            })
            .map(|(_, n)| n)
            .take_while(|&n| n <= INVALID_UTF32_HALF_START_MINUS_ONE)
            .map(|n| char::from_u32(n).expect("ehehe this is bad code"))
            .collect();
        let s = String::from_iter(&merged);
        
        match error {
            None => Ok(s),
            Some(i) => Err((s, i))
        }
    }
}


impl Encoding for Utf32Encoder {
    fn size(&self) -> usize {
        self.size
    }
}

impl Encoding for Utf32Decoder {
    fn size(&self) -> usize {
        self.size
    }
}
