use std::path::PathBuf;
use lexopt::prelude::*;

#[derive(Debug)]
pub struct Args {
    pub filename: PathBuf,
    pub read: bool,
    pub write: bool,
}

pub fn parse_args() -> Result<Args, lexopt::Error> {
    let mut filename_input = None;
    let mut read = false;
    let mut write = false;
    let mut parser = lexopt::Parser::from_env();

    while let Some(arg) = parser.next()? {
        match arg {
            Short('r') | Long("read") => read = true,
            Short('w') | Long("write") => write = true,
            Value(val) if filename_input.is_none() => {
                filename_input = Some(PathBuf::from(val));
            }
            Long("version") => {
                println!("id3-json {}", env!("CARGO_PKG_VERSION"));
                std::process::exit(0);
            }
            Long("help") => {
                println!("id3-json {}", env!("CARGO_PKG_VERSION"));
                println!("");
                println!("USAGE:");
                println!("    id3-image-json [FLAGS] <music-file.mp3>");
                println!("");
                println!("FLAGS:");
                println!("    -r, --read       Reads tags and outputs them as JSON.");
                println!("                     If neither `read` nor `write` are given, will read by default.");
                println!("    -w, --write      Write mode, will expect a JSON with valid tag values.");
                println!("                     If also given `read`, will print the resulting tags afterwards");
                println!("    -V, --version    Prints version information");
                println!("");
                println!("ARGS:");
                println!("    <music-file.mp3>    Music file to read tags from or write tags to");
                std::process::exit(0);
            }
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

    Ok(Args { filename, read, write })
}