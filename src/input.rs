use std::path::PathBuf;
use std::ffi::OsString;

use lexopt::prelude::*;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Args {
    pub filename: PathBuf,
    pub read: bool,
    pub write: bool,
    pub with_covers: bool,
    pub tag_version: Option<id3::Version>,
    pub in_json: Option<PathBuf>,
    pub out_json: Option<PathBuf>,
}

pub fn parse_args<I>(args: I) -> Result<Args, lexopt::Error>
where
    I: IntoIterator + 'static,
    I::Item: Into<OsString>,
{
    let mut read        = false;
    let mut write       = false;
    let mut with_covers = false;

    let mut filename_input = None;
    let mut tag_version    = None;
    let mut in_json        = None;
    let mut out_json       = None;

    let mut parser = lexopt::Parser::from_iter(args);

    while let Some(arg) = parser.next()? {
        match arg {
            Short('r') | Long("read")  => read        = true,
            Short('w') | Long("write") => write       = true,
            Long("with-covers")        => with_covers = true,

            Long("tag-version") => {
                let mut input = parser.value()?;
                input.make_ascii_lowercase();

                if input == "id3v2.2" {
                    tag_version = Some(id3::Version::Id3v22);
                } else if input == "id3v2.3" {
                    tag_version = Some(id3::Version::Id3v23);
                } else if input == "id3v2.4" {
                    tag_version = Some(id3::Version::Id3v24);
                } else {
                    let error = format!("Unsupported ID3 version: {:?}. Expected ID3v2.{{2,3,4}}", input);
                    return Err(lexopt::Error::Custom(error.into()));
                }
            },
            Value(val) if filename_input.is_none() => {
                filename_input = Some(PathBuf::from(val));
            },

            Short('i') | Long("in-json") => {
                let input = parser.value()?.into();
                in_json = Some(input);
            },
            Short('o') | Long("out-json") => {
                let input = parser.value()?.into();
                out_json = Some(input);
            },

            Short('V') | Long("version") => {
                println!("id3-json {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            },
            Long("help") => {
                print_help();
                std::process::exit(0);
            },
            _ => return Err(arg.unexpected()),
        }
    }

    let Some(filename) = filename_input else {
        let error = String::from("Missing <filename.mp3>");
        return Err(lexopt::Error::Custom(error.into()));
    };

    if !read && !write {
        read = true;
    }

    Ok(Args { filename, read, write, with_covers, tag_version, in_json, out_json })
}

fn print_help() {
    println!("id3-json {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("USAGE:");
    println!("    id3-json [FLAGS] <music-file.mp3>");
    println!();
    println!("FLAGS:");
    println!("    -r, --read       Reads tags from the file and outputs them to STDOUT as JSON,");
    println!("                     or writes them to the file given by --out-json.");
    println!("                     If neither `read` nor `write` are given, will read by default.");
    println!();
    println!("    -w, --write      Write mode, expects a JSON on STDIN with valid tag values,");
    println!("                     or reads the tags from the file given by --in-json.");
    println!("                     If also given `read`, will print/write the resulting tags afterwards");
    println!();
    println!("    --with-covers    Also output cover images as base64-encoded data.");
    println!("                     If not set, only cover metadata will be returned.");
    println!();
    println!("    -i, --in-json <path/to.json>");
    println!("                     File to read tags from. If not given, uses STDIN");
    println!();
    println!("    -o, --out-json <path/to.json>");
    println!("                     File to write tags to. If not given, uses STDOUT");
    println!();
    println!("        --tag-version <ID3v2.{{2,3,4}}>");
    println!("                     On write, sets the tags' version to 2.2, 2.3, or 2.4.");
    println!();
    println!("    -V, --version    Print version information");
    println!();
    println!("ARGS:");
    println!("    <music-file.mp3>    Music file to read tags from or write tags to");
}
