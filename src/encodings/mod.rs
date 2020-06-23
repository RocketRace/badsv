mod utf8;
mod utf16;

use utf8::{ Utf8Decoder, Utf8Encoder };
use utf16::{ Utf16Decoder, Utf16Encoder };
use std::fs::File;
use std::io::Read;

pub const VALID_ENCODINGS: [&str; 2] = ["utf-8", "utf-16"];

/// Result from attempt to decode bytes
pub type DecodeResult = Result<String, (String, usize)>;

pub trait Encoding {
    fn size(&self) -> usize;
}

pub trait Encoder {
    fn encode(&self, data: &str) -> Vec<u8>;
    fn get_delimiter(&self) -> Vec<u8>;
}

pub trait Decoder {
    fn try_decode(&self, bytes: &[u8]) -> DecodeResult;
}

/// Parses BadSV in the flavor provided
pub fn parse(data: &mut File, encoding: &str) -> Vec<Vec<String>> {
    match encoding {
        "utf-8" => parse_with(data, Utf8Decoder::new()),
        "utf-16" => parse_with(data, Utf16Decoder::new()),
        _ => panic!("Invalid encoding provided")
    }
}

/// Parses BadSV in the flavor provided
fn parse_with<E: Decoder + Encoding>(data: &mut File, decoder: E) -> Vec<Vec<String>> {
    let mut bytes = Vec::new();
    match data.read(&mut bytes) {
        Ok(_) => (),
        Err(_) => panic!("Could not read contents of file")
    }
    let mut out: Vec<Vec<String>> = Vec::new();
    let mut record: Vec<String> = Vec::new();
    let mut buffer: Vec<u8> = Vec::new();
    for chunk in bytes.chunks(decoder.size()) {
        buffer.extend(chunk);
        match decoder.try_decode(&buffer) {
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

pub fn compile(data: Vec<Vec<String>>, encoding: &str) -> Vec<u8> {
    match encoding {
        "utf-8" => compile_with(data, Utf8Encoder::new()),
        "utf-16" => compile_with(data, Utf16Encoder::new()),
        _ => panic!("Invalid encoding provided")
    }
}

/// Compiles a grid into a stream of BadSV bytes in the flavor provided
fn compile_with<E: Encoder + Encoding>(data: Vec<Vec<String>>, encoder: E) -> Vec<u8> {
    let mut out = Vec::new();
    for record in data {
        for (i, word) in record.iter().enumerate() {
            out.extend(encoder.encode(word));
            if i < record.len() - 1 {
                out.extend(encoder.get_delimiter());
            }
        }
        for _ in 0..(encoder.size() - 1) {
            out.push(0);
        }
        out.push('\n' as u8) // Sorry, Windows users
    }
    out
}