[![Build Status](https://circleci.com/gh/AndrewRadev/id3-json/tree/main.svg?style=shield)](https://circleci.com/gh/AndrewRadev/id3-json?branch=main)

# ID3-JSON

This project's goal is to provide an easy way to read and write ID3 tags with a consistent input and output. The existing tools I've found require a lot of work to parse their output and work in inconsistent ways, so I'm making another one.

The main client of this project is going to be a Vim plugin: <https://github.com/AndrewRadev/id3.vim>. That said, there's no particular reason not to use it for whatever. I'd like to keep the tool general-purpose, so if you're looking for some specific functionality, please open a github issue.

The project is backed by the excellent [id3](https://crates.io/crates/id3) crate built in Rust. Really, the tag parsing and writing logic is all there -- the code here mostly just translates the data to/from JSON (though I do make some assumptions, see [Quirks](#quirks) below).

## Basic usage

Running the program with `--help` should provide a message along these lines.

```
id3-json 0.1.0

USAGE:
    id3-json [FLAGS] <music-file.mp3>

FLAGS:
    -r, --read       Reads tags from the file and outputs them to STDOUT as JSON.
                     If neither `read` nor `write` are given, will read by default.

    -w, --write      Write mode, expects a JSON on STDIN with valid tag values.
                     If also given `read`, will print the resulting tags afterwards

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
    "genre": "(255)",
    "title": "Elevator Music Attempt #1",
    "track": null,
    "year": null
  }
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
    "genre": null,
    "title": "[updated]",
    "track": 1,
    "year": null
  }
}
```

## Quirks

The numbers given to the "year" field in [`set_year`](https://docs.rs/id3/1.5.1/id3/trait.TagLike.html#method.set_year) seem to be `i32`, but for simplicity, I assume years are going to be positive numbers.

It's possible to have multiple comments with a "description", "lang", and "text". See the [`frame::Comment`](https://docs.rs/id3/1.5.1/id3/frame/struct.Comment.html) structure for details. However, at least in my personal music library, it seems almost all mp3 files contain a single comment with `""` for the description. Some of them have another one that's labeled as `"ID3v1 comment"`.

For simplicity's sake I've decided to have `id3-json` read and write that one comment with a description of `""`. All other comments should be preserved, so if anything else reads them, it should still work as expected.

## Music used for testing:

Elevator Music Attempt 1 by Christian Bakker: <https://www.jamendo.com/en/list/a98147/echoes-from-the-past>
