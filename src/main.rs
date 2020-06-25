mod dsv;
mod encodings;

use std::fs::{File, OpenOptions};
use std::io::Write;
use clap::{App, Arg, ArgGroup, crate_version, crate_authors};

fn main() {
    // Command line arguments
    let args = App::new("BadSV File Converter")
        .about("Awesome(?) command-line tool for converting between DSV and BadSV files")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::from_usage("ascend -a --ascend 'Ascend from a DSV file into a BadSV file'")
        )
        .arg(
            Arg::from_usage("regress -r --regress 'Regress from a BadSV file into a DSV file (why would you do this?)'")
        )
        .arg(
            Arg::from_usage("list-encodings -l --list-encodings 'Lists valid BadSV encodings'")
        )
        .group(
            ArgGroup::with_name("action")
                .required(true)
                .args(&["ascend", "regress", "list-encodings"])
        )
        .arg(
            Arg::from_usage("encoding -e --encoding=[ENCODING] 'The flavor of BadSV used [Default: utf-8]'")
        )
        .arg(
            Arg::from_usage("delimiter -d --delimiter=[DELIMITER] 'The delimiters used in the DSV file [Default: ,]'")
        )
        .arg(
            Arg::from_usage("[input] 'Input file'")
        )
        .arg(
            Arg::from_usage("[output] 'Output file'")
        )
        .get_matches();

    let encoding = args.value_of("encoding").unwrap_or("utf-8");
    let delimiter = args.value_of("delimiter").unwrap_or(",");
    
    if args.is_present("list-encodings") {
        println!("Valid BadSV encodings:");
        for encoding in encodings::VALID_ENCODINGS.iter() {
            println!("* {}", encoding);
        }
    }
    else if args.is_present("ascend") {
        if let Some((input, output)) = get_io(&args) {
            if let Some(del) = validate_delimiter(delimiter) {
                let file = match File::open(input) {
                    Ok(f) => f,
                    Err(_) => panic!("Error opening file.")
                };
                let data = dsv::parse(file, del);
                let out = encodings::compile(data, encoding);
                write(&out, &output);
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
                let mut file = match File::open(input) {
                    Ok(f) => f,
                    Err(_) => panic!("Error opening file.")
                };
                let data = encodings::parse(&mut file, encoding);
                let out = dsv::compile(data, del);
                write(&out, &output);
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

/// Ensures the delimiter is one byte in length
fn validate_delimiter(delimiter: &str) -> Option<u8> {
    let bytes = delimiter.as_bytes();
    if bytes.len() == 1 {
        Some(bytes[0])
    }
    else {
        None
    }
}

/// Writes to a file (pretty inane documentation, huh)
fn write(bytes: &[u8], path: &str) {
    let mut options = OpenOptions::new();
    let mut file = match options.write(true).create(true).open(path) {
        Ok(f) => f,
        Err(_) => panic!("Error opening resulting file")
    };
    match file.write(bytes) {
        Ok(_) => (),
        Err(_) => panic!("Error writing file")
    };
}