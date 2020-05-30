mod dsv;
mod encodings;

use std::fs::File;
use clap::{App, Arg, ArgGroup, crate_version, crate_authors};

const VALID_ENCODINGS: [&str; 2] = ["utf-8", "utf-16"];

fn main() {
    // Command line arguments
    let args = App::new("BadSV File Converter")
        .about("Awesome(?) command-line tool for converting between DSV and BadSV files")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::from_usage("convert -c --convert 'Convert a DSV file into a BadSV file'")
        )
        .arg(
            Arg::from_usage("regress -r --regress 'Convert a BadSV file into a DSV file (why would you do this)'")
        )
        .arg(
            Arg::from_usage("list-encodings -l --list-encodings 'Lists valid BadSV encodings'")
        )
        .group(
            ArgGroup::with_name("action")
                .required(true)
                .args(&["convert", "regress", "list-encodings"])
        )
        .arg(
            Arg::from_usage("-s --source-encoding=[ENCODING] 'The encoding of the original file [Default: utf-8]'")
        )
        .arg(
            Arg::from_usage("-t --target-encoding=[ENCODING] 'The encoding of the resulting file [Default: utf-8]'")
        )
        .arg(
            Arg::from_usage("-d --delimiter=[DELIMITER] 'The delimiters used in the DSV file [Default: ,]'")
        )
        .arg(
            Arg::from_usage("[input] 'Input file'")
        )
        .arg(
            Arg::from_usage("[output] 'Output file'")
        )
        .get_matches();

    let source_encoding = args.value_of("source-encoding").unwrap_or("utf-8");
    let target_encoding = args.value_of("target-encoding").unwrap_or("utf-8");
    let delimiter = args.value_of("delimiter").unwrap_or(",");
    
    if args.is_present("list-encodings") {
        println!("Valid BadSV encodings:");
        for encoding in VALID_ENCODINGS.iter() {
            println!("* {}", encoding);
        }
    }
    else if args.is_present("convert") {
        if let Some((input, output)) = get_io(&args) {
            if let Some(del) = validate_delimiter(delimiter) {
                let file = match File::open(input) {
                    Ok(f) => f,
                    Err(_) => panic!("Error opening file.")
                };
                let data = dsv::parse(file, delimiter.as_bytes()[0]);
            }
            else {
                panic!("Delimiter must be 1 byte in length")
            }
        }
        else {
            panic!("Both input and output files must be provided.");
        }
    }
    else if args.is_present("regress") {
        if let Some((input, output)) = get_io(&args) {
            if let Some(del) = validate_delimiter(delimiter) {
                let file = match File::open(input) {
                    Ok(f) => f,
                    Err(_) => panic!("Error opening file.")
                };
            }
            else {
                panic!("Delimiter must be 1 byte in length")
            }
        }
        else {
            panic!("Both input and output files must be provided.");
        }
    }
    // Shouldn't get this far
}

/// Finds the input and output files from command-line matches, if found
fn get_io(args: &clap::ArgMatches) -> Option<(String, String)>{
    if let Some(input) = args.value_of("input") {
        if let Some(output) = args.value_of("output") {
            return Some((String::from(input), String::from(output)))
        }
    }
    None
}

// Ensures the delimiter is one byte in length
fn validate_delimiter(delimiter: &str) -> Option<u8> {
    let bytes = delimiter.as_bytes();
    if bytes.len() == 1 {
        Some(bytes[0])
    }
    else {
        None
    }
}