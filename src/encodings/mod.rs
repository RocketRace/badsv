mod utf8;
mod utf16_le;

use std::fs::File;
use std::io::Read;

pub const VALID_ENCODINGS: [&str; 2] = ["utf-8", "utf-16"];

/// Parses BadSV in the flavor provided
pub fn parse(data: &mut File, encoding: &str) -> Vec<Vec<String>> {
    let (try_decode, bytes_per) = match encoding {
        "utf-8" => (utf8::try_decode, utf8::MIN_BYTES),
        _ => panic!("Invalid encoding supplied")
    };

    let mut bytes = Vec::new();
    match data.read(&mut bytes) {
        Ok(_) => (),
        Err(_) => panic!("Could not read contents of file")
    }
    let mut out: Vec<Vec<String>> = Vec::new();
    let mut record: Vec<String> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();
    for chunk in bytes.chunks(bytes_per) {
        buffer.extend(chunk);
        match try_decode(&buffer) {
            Ok(s) =>  {
                // If a line break character is recognized
                if s.ends_with('\n') && chunk[chunk.len() - 1] == '\n' as u8 {
                    out.push(record.clone());
                    record.clear();
                }
            },
            Err((valid, _)) => {
                record.push(valid);
                buffer.clear();
            }
        }
    }
    out
}

/// Compiles a grid into a stream of BadSV bytes in the flavor provided
pub fn compile(data: Vec<Vec<String>>, encoding: &str) -> Vec<u8> {
    let (encode, get_delimiter, bytes_per) = match encoding {
        "utf-8" => (utf8::encode, utf8::get_delimiter, utf8::MIN_BYTES),
        _ => panic!("Invalid encoding supplied")
    };
    let mut out = Vec::new();
    for record in data {
        for (i, word) in record.iter().enumerate() {
            out.extend(encode(word));
            if i < record.len() - 1 {
                out.extend(get_delimiter());
            }
        }
        for _ in 0..(bytes_per - 1) {
            out.push(0);
        }
        out.push('\n' as u8) // Sorry, Windows users
    }
    out
}