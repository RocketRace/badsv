mod dsv;
mod encodings;

use clap::{App, Arg, ArgGroup, crate_version, crate_authors};

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
    if let Some(input) = args.value_of("input") {
        if let Some(output) = args.value_of("output") {
            println!("{}", source_encoding);
            println!("{}", target_encoding);
            println!("{}", delimiter);
            println!("{}", input);
            println!("{}", output);
        }
        else {
            println!("gonna eat you");
        }
    }
    else {
        println!("gonna yeet you");
    }
}