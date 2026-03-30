use std::path::PathBuf;

use id3_json::input::*;

#[test]
fn test_basic_inputs() {
    let args = parse_args(&["id3-json", "filename.mp3"]).unwrap();
    assert_eq!(args, Args { filename: PathBuf::from("filename.mp3"), read: true, ..Args::default() });

    let args = parse_args(&["id3-json", "filename.mp3", "--write"]).unwrap();
    assert_eq!(args, Args { filename: PathBuf::from("filename.mp3"), write: true, ..Args::default() });

    let args = parse_args(&["id3-json", "filename.mp3", "--read", "--write"]).unwrap();
    assert_eq!(args, Args { filename: PathBuf::from("filename.mp3"), read: true, write: true, ..Args::default() });

    let args = parse_args(&["id3-json", "filename.mp3", "-rw"]).unwrap();
    assert_eq!(args, Args { filename: PathBuf::from("filename.mp3"), read: true, write: true, ..Args::default() });

    let args = parse_args(&["id3-json", "-w", "filename.mp3"]).unwrap();
    assert_eq!(args, Args { filename: PathBuf::from("filename.mp3"), write: true, ..Args::default() });

    let args = parse_args(&["id3-json", "filename.mp3", "--tag-version", "ID3v2.4"]).unwrap();
    assert_eq!(args, Args {
        filename:    PathBuf::from("filename.mp3"),
        read:        true,
        tag_version: Some(id3::Version::Id3v24),
        ..Args::default()
    });

    let args = parse_args(&["id3-json", "filename.mp3", "--in-json", "some/path.json"]).unwrap();
    assert_eq!(args, Args {
        filename: PathBuf::from("filename.mp3"),
        read:     true,
        in_json:  Some(PathBuf::from("some/path.json")),
        ..Args::default()
    });

    let args = parse_args(&[
        "id3-json", "-w", "filename.mp3",
        "-i", "some/path.json",
        "-o", "other/path.json",
    ]).unwrap();
    assert_eq!(args, Args {
        filename: PathBuf::from("filename.mp3"),
        write:    true,
        in_json:  Some(PathBuf::from("some/path.json")),
        out_json: Some(PathBuf::from("other/path.json")),
        ..Args::default()
    });
}

#[test]
fn test_invalid_inputs() {
    let args = parse_args(&["id3-json"]);
    assert!(args.is_err());
    assert_eq!(format!("{}", args.unwrap_err()), "Missing <filename.mp3>");

    let args = parse_args(&["id3-json", "filename.mp3", "--unexpected"]);
    assert!(args.is_err());
    assert_eq!(format!("{}", args.unwrap_err()), "invalid option '--unexpected'");

    let args = parse_args(&["id3-json", "filename.mp3", "--tag-version", "foobar"]);
    assert!(args.is_err());
    assert_eq!(format!("{}", args.unwrap_err()), "Unsupported ID3 version: \"foobar\". Expected ID3v2.{2,3,4}");
}
