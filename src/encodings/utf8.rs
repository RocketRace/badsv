use rand::seq::SliceRandom;
use rand::thread_rng;

const INVALID_UTF8_BYTES: [u8; 13] = [
    // Bytes that are invalid anywhere in a utf-8 string
    192, 193, // c0, c1
    245, 246, 247, 248, 249, 250, 251, 252, 253, 254, 255 // f5 ~ ff
    // Bytes that are unexpected at the start of a byte sequence
];

/// Generates a random invalid utf-8 byte
pub fn get_delimiter() -> u8 {
    *INVALID_UTF8_BYTES.choose(&mut thread_rng()).unwrap()
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
