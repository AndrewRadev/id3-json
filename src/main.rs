use std::path::PathBuf;

use anyhow::anyhow;
use id3::TagLike;
use serde::{Serialize, Deserialize};

#[derive(Debug)]
struct Args {
    filename: PathBuf,
    read: bool,
    write: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TagData<'a> {
    title: Option<&'a str>,
    artist: Option<&'a str>,
    album: Option<&'a str>,
    track: Option<u32>,
    genre: Option<&'a str>,
    comments: Vec<String>,
}

fn main() -> anyhow::Result<()> {
    let args = parse_args()?;
    let mut tag = id3::Tag::read_from_path(&args.filename)?;
    if args.write {
        let new_tags: serde_json::Map<String, serde_json::Value> =
            serde_json::from_reader(std::io::stdin())?;

        for (key, value) in new_tags {
            match key.as_str() {
                "title" => tag.set_title(expect_string("title", value)?),
                _ => todo!(),
            }
        }

        tag.write_to_path(&args.filename, id3::Version::Id3v23)?;
    }

    let tag_data = TagData {
        title: tag.title(),
        artist: tag.artist(),
        album: tag.album(),
        track: tag.track(),
        genre: tag.genre(),
        comments: tag.comments().map(|c| c.to_string()).collect(),
    };

    if args.read {
        serde_json::to_writer(std::io::stdout(), &tag_data)?;
    }

    Ok(())
}

fn parse_args() -> Result<Args, lexopt::Error> {
    use lexopt::prelude::*;

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

fn expect_string(label: &str, json_value: serde_json::Value) -> anyhow::Result<String> {
    json_value.
        as_str().
        map(String::from).
        ok_or_else(|| anyhow!("Invalid value for \"{}\": {:?}", label, json_value))
}
