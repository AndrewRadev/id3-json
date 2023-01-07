[![Build Status](https://circleci.com/gh/AndrewRadev/id3-json/tree/main.svg?style=shield)](https://circleci.com/gh/AndrewRadev/id3-json?branch=main)

# ID3-JSON

This project's goal is to provide an easy way to read and write ID3 tags with a consistent input and output. The existing tools I've found require a lot of work to parse their output and work in inconsistent ways, so I'm making another one.

The main client of this project is going to be a Vim plugin: <https://github.com/AndrewRadev/id3.vim>. That said, there's no particular reason not to use it for whatever. I'd like to keep the tool general-purpose, so if you're looking for some specific functionality, please open a github issue.

The project is backed by the excellent [id3](https://crates.io/crates/id3) crate built in Rust. Really, the tag parsing and writing logic is all there -- the code here mostly just translates the data to/from JSON (though I do make some assumptions, see [Quirks](#quirks) below).

## Installation

If you have the Rust toolchain installed, you can install it from crates.io:

```
$ cargo install id3-json
```

But you can also use the precompiled binary for your operating system from the releases tab in github: <https://github.com/AndrewRadev/id3-json/releases>:

- Linux: [binary](https://github.com/AndrewRadev/id3-json/releases/download/v0.2.0/id3-json_v0.2.0_x86_64-unknown-linux-musl.zip), [sha256 checksum](https://github.com/AndrewRadev/id3-json/releases/download/v0.2.0/id3-json_v0.2.0_x86_64-unknown-linux-musl.zip.sha256sum)
- Windows: [binary](https://github.com/AndrewRadev/id3-json/releases/download/v0.2.0/id3-json_v0.2.0_x86_64-pc-windows-gnu.zip), [sha256 checksum](https://github.com/AndrewRadev/id3-json/releases/download/v0.2.0/id3-json_v0.2.0_x86_64-pc-windows-gnu.zip.sha256sum)
- Mac: [binary](https://github.com/AndrewRadev/id3-json/releases/download/v0.2.0/id3-json_v0.2.0_x86_64-apple-darwin.zip), [sha256 checksum](https://github.com/AndrewRadev/id3-json/releases/download/v0.2.0/id3-json_v0.2.0_x86_64-apple-darwin.zip.sha256sum)

## Basic usage

Running the program with `--help` should provide a message along these lines.

```
id3-json 0.2.0

USAGE:
    id3-json [FLAGS] <music-file.mp3>

FLAGS:
    -r, --read       Reads tags from the file and outputs them to STDOUT as JSON.
                     If neither `read` nor `write` are given, will read by default.

    -w, --write      Write mode, expects a JSON on STDIN with valid tag values.
                     If also given `read`, will print the resulting tags afterwards

        --tag-version <ID3v2.{2,3,4}>
                     On write, sets the tags' version to 2.2, 2.3, or 2.4.

    -V, --version    Prints version information

ARGS:
    <music-file.mp3>    Music file to read tags from or write tags to
```

The input to write to a tag should be a valid json with "title", "artist", etc as keys. The output will be a JSON object that has a "data" key and inside has all these fields set. Here's some example output, pretty-printed using the [jq](https://stedolan.github.io/jq/) tool:

``` .sh-session
% id3-json tests/fixtures/attempt_1.mp3 | jq .
{
  "data": {
    "album": "Echoes From The Past",
    "artist": "Christiaan Bakker",
    "comment": "http://www.jamendo.com Attribution 3.0 ",
    "date": null,
    "genre": "(255)",
    "title": "Elevator Music Attempt #1",
    "track": null
  },
  "version": "ID3v2.4"
}
```

Here's how we can update the title and track number, and remove the genre. The tool will print the tags after the change (because of `--read`):

``` .sh-session
% echo '{ "title": "[updated]", "track": 1, "genre": null }' | id3-json tests/fixtures/attempt_1.mp3 --write --read | jq .
{
  "data": {
    "album": "Echoes From The Past",
    "artist": "Christiaan Bakker",
    "comment": "http://www.jamendo.com Attribution 3.0 ",
    "date": null,
    "genre": null,
    "title": "[updated]",
    "track": 1
  },
  "version": "ID3v2.4"
}
```

## Quirks

The numbers given to the "year" field in [`set_year`](https://docs.rs/id3/1.5.1/id3/trait.TagLike.html#method.set_year) seem to be `i32`, but for simplicity, I assume years are going to be positive numbers.

If the tags are ID3v2.4, the tool will read and write the "date" field as "date recorded" (TDRL). See the relevant github issue for the conversation: <https://github.com/AndrewRadev/id3-json/issues/1>. Seems like both picard and easy tag use that field, which is why I chose it.

It's still a bit up in the air, I might end up dealing with both "date recorded" and "date released" as separate fields, though I can't help but wonder how many people care and would be happy to just have "date". For a tag that's not v2.4, it'll return "year" from the TYER tag instead.

It's possible to have multiple comments with a "description", "lang", and "text". See the [`frame::Comment`](https://docs.rs/id3/1.5.1/id3/frame/struct.Comment.html) structure for details. However, at least in my personal music library, it seems almost all mp3 files contain a single comment with `""` for the description. Some of them have another one that's labeled as `"ID3v1 comment"`.

For simplicity's sake I've decided to have `id3-json` read and write that one comment with a description of `""`. All other comments should be preserved, so if anything else reads them, it should still work as expected.

## Potential future changes

For now, this interface works for me, but if other people need to use it without some of my design choices, a `--raw` or `--full` option could be implemented to just read and write the frames as-is with minimal processing and leave it to the client of the tool to decide how to manage them.

Batch processing is another direction I could take this, returning a JSON array with an entry for each filename (or an object with filenames as keys?) and, when writing, expecting a corresponding array/object.

A lot of other metadata could also be read/written, the specific fields I've chosen are just what I used to use from a different utility.

## Music used for testing:

- Elevator Music Attempt 1 by Christian Bakker: <https://www.jamendo.com/en/list/a98147/echoes-from-the-past>
- The Masochism Tango by Tom Lehrer: <https://tomlehrersongs.com/the-masochism-tango/>
