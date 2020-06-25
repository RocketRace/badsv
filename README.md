# BaDSV — BaDly-Separated Values 

*The evolution of CSV & other DSV files—BaDSV is what you get when you leave your CSV file in the oven for too long. The delicious charred scent
of your data being stored in a readable, compact, and handsome format...*

BaDSV is a family of text-based file formats designed to store values, seperated by delimiters. **BaDSV files are delimited by random invalid byte sequences**,
defined as a constant-length string of bytes that are impossible at the beginning of a code unit, for a given character encoding.

This following example demonstrates the conversion process between CSV and utf-8-flavored BaDSV files:

``` 
foo,bar   | <---- CSV |    foo�bar
a,"b,"    | BaDSV --> |    a�b,
```

Several flavors of BaDSV are currently supported, each with their own set of randomly-generated delimiters:

| Flavor | Delimiter size | Possible Delimiters                               |
|--------|----------------|---------------------------------------------------|
| utf-8  | 1 byte         | `0xc0` ~ `0xc1`, `0xf5` ~ `0xff`, `0x80` ~ `0xbf` |
| utf-16 | 2 bytes        | `0xd800` ~ `0xdfff`                               |

## Pros of BaDSV

* No need to quote values, since delimiters physically can't be part of the values
* Marginally smaller file size compared to DSV files
* Doesn't lose information in the conversion process (usually)
* Cool and good

## Cons of BaDSV (don't be fooled, these are all very minor)

* Difficult to work with in many text renderers
* Often unreadable
* Not cross-platform
* Equivalent files aren't equal to the byte
* Character encoding is tied to the file format
* A colossal headache to implement APIs for
* Unnecessary reliance on randomness
* Has the same limitations as DSV files, only in a different jacket
* No quote support leads to myriad potential edge cases
* Not fully text-based, but not fully binary serialized - possibly the worst of both worlds
* Impossible to determine whether a byte is part of a delimiter without parsing the whole file until the byte
* Is *not* a format that has been developed, standardized, and polished to a respectable state throughout the years

# The BaDSV command-line tool

Compile this repository using cargo to gain access to a heavenly BaDSV file converter. 
The CLI is exquisitely crafted using the most elaborate default settings available, and
the error messages are enlightnening.

## Usage (resemblance to --help output is purely coincidental)

```
USAGE:
    badsv [OPTIONS] <--ascend|--regress|--list-encodings> [ARGS]

FLAGS:
    -a, --ascend            Ascend from a DSV file into a BadSV file
    -h, --help              Prints help information
    -l, --list-encodings    Lists valid BadSV encodings
    -r, --regress           Regress from a BadSV file into a DSV file (why would you do this?)
    -V, --version           Prints version information

OPTIONS:
    -d, --delimiter <DELIMITER>    The delimiters used in the DSV file [Default: ,]
    -e, --encoding <ENCODING>      The flavor of BadSV used [Default: utf-8]

ARGS:
    <input>     Input file
    <output>    Output file
```

# FAQ

## Why?

Why not?

## What's wrong with CSV?

It's not fancy, fashionable or even remotely handsome. What isn't wrong with CSV?

## Why do you think this will supercede CSV?

People like to roll the dice. BaDSV makes use of that primal need, and incorporates randomness to a rigid file format.

## Aren't these just C strings on meth?

Yes

## Why does this even exist?

I had a moment of enlightenment, and realized what must be done.

## [Something mean about code quality]

Look. Documentation, testing and other good programming habits are overblown. BaDSV transcends testing.

## Rust btw

Rust btw
