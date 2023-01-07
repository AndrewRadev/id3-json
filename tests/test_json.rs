use serde_json::json;

use id3_json::json::*;

mod support;
use support::fixture::Fixture;
use support::tag::read_tag;

#[test]
fn test_read_from_tag() {
    let song = Fixture::copy("attempt_1.mp3");
    let tag = read_tag(&song);
    let json = read_from_tag(&tag);

    assert_eq!(json.get("data").unwrap().get("title").unwrap(), "Elevator Music Attempt #1");
    assert_eq!(json.get("data").unwrap().get("artist").unwrap(), "Christiaan Bakker");
    assert_eq!(json.get("data").unwrap().get("comment").unwrap(), "http://www.jamendo.com Attribution 3.0 ");
}

#[test]
fn test_partial_write_to_tag() {
    let song = Fixture::copy("attempt_1.mp3");
    let mut tag = read_tag(&song);

    let new_data = json!({
        "title": "New title",
        "comment": "New comment",
    }).as_object().unwrap().clone();

    write_to_tag(new_data, &mut tag, None).unwrap();
    let json = read_from_tag(&tag);

    assert_eq!(json.get("data").unwrap().get("title").unwrap(), "New title");
    assert_eq!(json.get("data").unwrap().get("artist").unwrap(), "Christiaan Bakker");
    assert_eq!(json.get("data").unwrap().get("comment").unwrap(), "New comment");
}

#[test]
fn test_full_write_to_tag() {
    let song = Fixture::copy("attempt_1.mp3");
    let mut tag = read_tag(&song);

    let new_data = json!({
        "title": "New title",
        "artist": "ID3-JSON",
        "album": "Songs of data processing",
        "track": 1337,
        "date": "2022-12-12",
        "genre": "Electronic",
        "comment": "No comment",
    }).as_object().unwrap().clone();

    write_to_tag(new_data, &mut tag, None).unwrap();
    let json = read_from_tag(&tag);

    assert_eq!(json.get("data").unwrap().get("title").unwrap(), "New title");
    assert_eq!(json.get("data").unwrap().get("artist").unwrap(), "ID3-JSON");
    assert_eq!(json.get("data").unwrap().get("album").unwrap(), "Songs of data processing");
    assert_eq!(json.get("data").unwrap().get("track").unwrap(), 1337);
    assert_eq!(json.get("data").unwrap().get("date").unwrap(), "2022-12-12");
    assert_eq!(json.get("data").unwrap().get("genre").unwrap(), "Electronic");
    assert_eq!(json.get("data").unwrap().get("comment").unwrap(), "No comment");
}

#[test]
fn test_tag_removal() {
    let song = Fixture::copy("attempt_1.mp3");
    let mut tag = read_tag(&song);

    let new_data = json!({
        "title": None::<String>,
        "comment": None::<String>,
    }).as_object().unwrap().clone();

    write_to_tag(new_data, &mut tag, None).unwrap();
    let json = read_from_tag(&tag);

    assert_eq!(json.get("data").unwrap().get("title").unwrap(), &serde_json::Value::Null);
    assert_eq!(json.get("data").unwrap().get("artist").unwrap(), "Christiaan Bakker");
    assert_eq!(json.get("data").unwrap().get("comment").unwrap(), &serde_json::Value::Null);
}

#[test]
fn test_multiple_comments_1() {
    use id3::frame::{Content, Comment};
    use id3::Frame;
    use id3::TagLike;

    let mut tag = id3::Tag::new();
    assert_eq!(tag.comments().count(), 0);

    let json = read_from_tag(&tag);
    assert_eq!(json.get("data").unwrap().get("comment").unwrap(), &serde_json::Value::Null);

    let frame = Frame::with_content("COMM", Content::Comment(Comment {
        lang: "eng".to_owned(),
        description: "key1".to_owned(),
        text: "value1".to_owned()
    }));
    tag.add_frame(frame);
    assert_eq!(tag.comments().count(), 1);

    // Ignored, we only take comments with the description "":
    let json = read_from_tag(&tag);
    assert_eq!(json.get("data").unwrap().get("comment").unwrap(), &serde_json::Value::Null);

    let frame = Frame::with_content("COMM", Content::Comment(Comment {
        lang: "".to_owned(),
        description: "".to_owned(),
        text: "value2".to_owned()
    }));
    tag.add_frame(frame);
    assert_eq!(tag.comments().count(), 2);

    let json = read_from_tag(&tag);
    assert_eq!(json.get("data").unwrap().get("comment").unwrap(), "value2");

    // Update "" comment:
    let new_data = json!({ "comment": "updated value2" }).as_object().unwrap().clone();

    write_to_tag(new_data, &mut tag, None).unwrap();
    assert_eq!(tag.comments().count(), 2);

    let json = read_from_tag(&tag);
    assert_eq!(json.get("data").unwrap().get("comment").unwrap(), "updated value2");

    // Remove "" comment, check that the other is still there:
    let new_data = json!({ "comment": None::<String> }).as_object().unwrap().clone();

    write_to_tag(new_data, &mut tag, None).unwrap();
    assert_eq!(tag.comments().count(), 1);

    let json = read_from_tag(&tag);

    assert_eq!(json.get("data").unwrap().get("comment").unwrap(), &serde_json::Value::Null);
    assert_eq!(tag.comments().next().unwrap().text, "value1");
}

#[test]
fn test_nul_byte_at_the_end_of_comment() {
    let song = Fixture::copy("attempt_1.mp3");
    let mut tag = read_tag(&song);

    let new_data = json!({
        "comment": "New comment\u{0000}",
    }).as_object().unwrap().clone();

    write_to_tag(new_data, &mut tag, None).unwrap();
    let json = read_from_tag(&tag);

    assert_eq!(json.get("data").unwrap().get("comment").unwrap(), "New comment");
}

#[test]
fn test_multiple_comments_2() {
    use id3::frame::{Content, Comment};
    use id3::Frame;
    use id3::TagLike;

    let mut tag = id3::Tag::new();
    assert_eq!(tag.comments().count(), 0);

    let frame = Frame::with_content("COMM", Content::Comment(Comment {
        lang: "eng".to_owned(),
        description: "key1".to_owned(),
        text: "value1".to_owned()
    }));
    tag.add_frame(frame);
    assert_eq!(tag.comments().count(), 1);

    // Add new "" comment:
    let new_data = json!({ "comment": "value2" }).as_object().unwrap().clone();

    write_to_tag(new_data, &mut tag, None).unwrap();
    assert_eq!(tag.comments().count(), 2);

    let json = read_from_tag(&tag);
    assert_eq!(json.get("data").unwrap().get("comment").unwrap(), "value2");
}

#[test]
fn test_date_from_id3v24_tag() {
    use id3::TagLike;

    let song = Fixture::copy("attempt_1.mp3");
    let mut tag = read_tag(&song);

    tag.set_date_recorded(id3::Timestamp {
        year:   2023,
        month:  Some(6),
        day:    None,
        hour:   None,
        minute: None,
        second: None,
    });
    tag.write_to_path(&*song, id3::Version::Id3v24).unwrap();

    let tag = read_tag(&song);
    let json = read_from_tag(&tag);

    assert_eq!(json.get("data").unwrap().get("date").unwrap(), "2023-06");
    assert_eq!(json.get("data").unwrap().get("year"), None);
}

#[test]
fn test_year_from_id3v23_tag() {
    let song = Fixture::copy("10. the masochism tango.mp3");
    let tag = read_tag(&song);
    tag.write_to_path(&*song, id3::Version::Id3v23).unwrap();

    let mut tag = read_tag(&song);
    let json = read_from_tag(&tag);

    // Reading and writing the year works:
    assert_eq!(json.get("data").unwrap().get("year").unwrap(), &serde_json::Value::Null);

    let new_data = json!({ "year": "2023" }).as_object().unwrap().clone();
    write_to_tag(new_data, &mut tag, None).unwrap();
    let json = read_from_tag(&tag);
    assert_eq!(json.get("data").unwrap().get("year").unwrap(), 2023);

    // Writing a date doesn't work:
    let new_data = json!({ "date": "2023-06-01" }).as_object().unwrap().clone();
    write_to_tag(new_data, &mut tag, None).unwrap();
    let json = read_from_tag(&tag);
    assert_eq!(json.get("data").unwrap().get("date"), None);
}
