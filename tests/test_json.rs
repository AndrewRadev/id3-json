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

    write_to_tag(new_data, &mut tag).unwrap();
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
        "year": 2022,
        "genre": "Electronic",
        "comment": "No comment",
    }).as_object().unwrap().clone();

    write_to_tag(new_data, &mut tag).unwrap();
    let json = read_from_tag(&tag);

    assert_eq!(json.get("data").unwrap().get("title").unwrap(), "New title");
    assert_eq!(json.get("data").unwrap().get("artist").unwrap(), "ID3-JSON");
    assert_eq!(json.get("data").unwrap().get("album").unwrap(), "Songs of data processing");
    assert_eq!(json.get("data").unwrap().get("track").unwrap(), 1337);
    assert_eq!(json.get("data").unwrap().get("year").unwrap(), 2022);
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

    write_to_tag(new_data, &mut tag).unwrap();
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

    write_to_tag(new_data, &mut tag).unwrap();
    assert_eq!(tag.comments().count(), 2);

    let json = read_from_tag(&tag);
    assert_eq!(json.get("data").unwrap().get("comment").unwrap(), "updated value2");

    // Remove "" comment, check that the other is still there:
    let new_data = json!({ "comment": None::<String> }).as_object().unwrap().clone();

    write_to_tag(new_data, &mut tag).unwrap();
    assert_eq!(tag.comments().count(), 1);

    let json = read_from_tag(&tag);

    assert_eq!(json.get("data").unwrap().get("comment").unwrap(), &serde_json::Value::Null);
    assert_eq!(tag.comments().next().unwrap().text, "value1");
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

    write_to_tag(new_data, &mut tag).unwrap();
    assert_eq!(tag.comments().count(), 2);

    let json = read_from_tag(&tag);
    assert_eq!(json.get("data").unwrap().get("comment").unwrap(), "value2");
}

#[test]
fn test_year_fallback() {
    use id3::TagLike;

    let song = Fixture::copy("attempt_1.mp3");
    let mut tag = read_tag(&song);

    tag.remove_year();
    tag.set_date_released(id3::Timestamp {
        year:   2023,
        month:  Some(6),
        day:    None,
        hour:   None,
        minute: None,
        second: None,
    });
    tag.write_to_path(&*song, id3::Version::Id3v23).unwrap();

    let mut tag = read_tag(&song);
    let json = read_from_tag(&tag);

    assert_eq!(json.get("data").unwrap().get("year").unwrap(), 2023);

    tag.remove_date_released();
    tag.set_original_date_released(id3::Timestamp {
        year:   1990,
        month:  Some(7),
        day:    Some(28),
        hour:   None,
        minute: None,
        second: None,
    });
    tag.write_to_path(&*song, id3::Version::Id3v23).unwrap();

    let mut tag = read_tag(&song);
    let json = read_from_tag(&tag);

    assert_eq!(json.get("data").unwrap().get("year").unwrap(), 1990);

    tag.remove_original_date_released();
    tag.set_date_recorded(id3::Timestamp {
        year:   1989,
        month:  Some(5),
        day:    Some(23),
        hour:   None,
        minute: None,
        second: None,
    });
    tag.write_to_path(&*song, id3::Version::Id3v23).unwrap();

    let tag = read_tag(&song);
    let json = read_from_tag(&tag);

    assert_eq!(json.get("data").unwrap().get("year").unwrap(), 1989);
}
