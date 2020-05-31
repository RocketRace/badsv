use rand::seq::SliceRandom;
use rand::thread_rng;

pub const MIN_BYTES: usize = 1;

const INVALID_UTF8_START_BYTES: [u8; 77] = [
    // Bytes that are invalid anywhere in a utf-8 string
    0xc0, 0xc1,
    0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb, 0xfc, 0xfd, 0xfe, 0xff, 
    // Bytes that are unexpected at the start of a byte sequence
    0x80, 0x81, 0x82, 0x83, 0x84, 0x85, 0x86, 0x87,
    0x88, 0x89, 0x8a, 0x8b, 0x8c, 0x8d, 0x8e, 0x8f, 
    0x90, 0x91, 0x92, 0x93, 0x94, 0x95, 0x96, 0x97,
    0x98, 0x99, 0x9a, 0x9b, 0x9c, 0x9d, 0x9e, 0x9f, 
    0xa0, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7,
    0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae, 0xaf, 
    0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7,
    0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe, 0xbf, 
];

/// Generates a random invalid utf-8 byte
pub fn get_delimiter() -> Vec<u8> {
    vec![*INVALID_UTF8_START_BYTES.choose(&mut thread_rng()).unwrap()]
}

/// Decode bytes from a utf-8 String as far as possible
pub fn try_decode(bytes: &[u8]) -> Result<String, (String, usize)> {
    let out = String::from_utf8(bytes.to_vec());
    match out {
        Ok(s) => Ok(s),
        Err(why) => {
            let error = why.utf8_error();
            let until = error.valid_up_to();
            // Safe because we've ensured the unchecked bytes are valid already
            let valid = unsafe {
                String::from_utf8_unchecked(bytes[..until].to_vec())
            };
            if let Some(_) = error.error_len() {
                Err((valid, until))
            }
            else {
                // Give me more bytes
                Ok(valid)
            }
        }
    }
}

/// Encodes a utf-8 string
pub fn encode(data: &str) -> Vec<u8> {
    data.as_bytes().to_vec()
}
