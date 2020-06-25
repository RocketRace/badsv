use csv;

use std::fs::File;

/// Parses the contents of a DSV file, and returns 
/// the grid of items given the delimiter used.
pub fn parse(data: File, delimiter: u8) -> Vec<Vec<String>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .has_headers(false)
        .flexible(true)
        .from_reader(data);
    let out = reader
        .records()
        .map(
            |r| r.unwrap().iter().map(|s| String::from(s)).collect()
        )
        .collect::<Vec<Vec<String>>>();
    out
}

/// Compiles a grid of items into a stream of bytes.
pub fn compile(data: Vec<Vec<String>>, delimiter: u8) -> Vec<u8> {
    let mut writer = csv::WriterBuilder::new()
        .delimiter(delimiter)
        .flexible(true)
        .from_writer(Vec::new());

    for record in data {
        writer.write_record(record).unwrap();
    }

    writer.into_inner().unwrap()
    // All these unwrap()s are justified because I say so
}