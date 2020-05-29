use csv;

use std::fs::File;

/// Parses the contents of a DSV file, and returns 
/// the grid of items given the delimiter used.
pub fn parse(data: File, delimiter: u8) -> Vec<Vec<String>> {
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(delimiter)
        .from_reader(data);
    let out = reader
        .records()
        .map(
            |r| r.unwrap().iter().map(|s| String::from(s)).collect()
        )
        .collect::<Vec<Vec<String>>>();
    out
}