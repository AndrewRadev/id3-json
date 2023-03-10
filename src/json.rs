use anyhow::anyhow;
use id3::TagLike;

pub fn read_from_tag(tag: &id3::Tag) -> serde_json::Value {
    // There could be many comments, but in my music library, it seems like it's common to just
    // have one with a "description" set to an empty string. So let's have a single "comment" field
    // that reads and writes there.
    let comment = tag.comments().
        find(|c| c.description.is_empty()).
        map(|c| remove_nul_byte(&c.text).to_string());

    if tag.version() == id3::Version::Id3v24 {
        serde_json::json!({
            "version": format!("{}", tag.version()),
            "data": {
                "title": tag.title().map(remove_nul_byte),
                "artist": tag.artist().map(remove_nul_byte),
                "album": tag.album().map(remove_nul_byte),
                "track": tag.track(),
                "date": tag.date_recorded().map(|ts| format!("{}", ts)),
                "genre": tag.genre().map(remove_nul_byte),
                "comment": comment,
            },
        })
    } else {
        serde_json::json!({
            "version": format!("{}", tag.version()),
            "data": {
                "title": tag.title().map(remove_nul_byte),
                "artist": tag.artist().map(remove_nul_byte),
                "album": tag.album().map(remove_nul_byte),
                "track": tag.track(),
                "year": tag.year(),
                "genre": tag.genre().map(remove_nul_byte),
                "comment": comment,
            },
        })
    }
}

pub fn write_to_tag(
    json_map: serde_json::Map<String, serde_json::Value>,
    tag: &mut id3::Tag,
    version: Option<id3::Version>,
) -> anyhow::Result<()> {
    let version = version.unwrap_or_else(|| tag.version());

    for (key, value) in json_map {
        match key.as_str() {
            "title" => {
                if let Some(title) = extract_string("title", &value)? {
                    tag.set_title(title);
                } else {
                    tag.remove_title();
                }
            },
            "artist" => {
                if let Some(artist) = extract_string("artist", &value)? {
                    tag.set_artist(artist);
                } else {
                    tag.remove_artist();
                }
            },
            "album" => {
                if let Some(album) = extract_string("album", &value)? {
                    tag.set_album(album);
                } else {
                    tag.remove_album();
                }
            },
            "track" => {
                if let Some(track) = extract_u32("track", &value)? {
                    tag.set_track(track);
                } else {
                    tag.remove_track();
                }
            },
            "year" if version < id3::Version::Id3v24 => {
                if let Some(year) = extract_u32("year", &value)? {
                    tag.set_year(year.try_into()?);
                } else {
                    tag.remove_year();
                }
            },
            "date" if version >= id3::Version::Id3v24 => {
                if let Some(date) = extract_string("date", &value)? {
                    tag.set_date_recorded(date.parse()?);
                } else {
                    tag.remove_date_recorded();
                }
            },
            "genre" => {
                if let Some(genre) = extract_string("genre", &value)? {
                    tag.set_genre(genre);
                } else {
                    tag.remove_genre();
                }
            },
            "comment" => {
                let mut comment_frames = tag.remove("COMM");
                let existing_index = comment_frames.iter().
                    position(|c| c.content().comment().unwrap().description.is_empty());
                let new_comment_body = extract_string("comment", &value)?;

                match (existing_index, new_comment_body) {
                    (Some(index), None) => {
                        comment_frames.remove(index);
                    },
                    (Some(index), Some(text)) => {
                        let existing_comment = comment_frames[index].content().comment().unwrap();
                        let mut new_comment = existing_comment.clone();
                        new_comment.text = text;

                        let new_frame = id3::Frame::with_content("COMM", id3::Content::Comment(new_comment));
                        comment_frames[index] = new_frame;
                    },
                    (None, Some(text)) => {
                        let new_comment = id3::frame::Comment {
                            lang: String::new(),
                            description: String::new(),
                            text,
                        };
                        let new_frame = id3::Frame::with_content("COMM", id3::Content::Comment(new_comment));

                        comment_frames.push(new_frame);
                    }
                    (None, None) => continue,
                }

                for frame in comment_frames {
                    tag.add_frame(frame);
                }
            },
            _ => (),
        }
    }

    Ok(())
}


fn extract_string(label: &str, json_value: &serde_json::Value) -> anyhow::Result<Option<String>> {
    match json_value {
        serde_json::Value::Null          => Ok(None),
        serde_json::Value::String(value) => Ok(Some(value.clone())),
        _ => Err(anyhow!("Invalid string value for \"{}\": {:?}", label, json_value)),
    }
}

fn extract_u32(label: &str, json_value: &serde_json::Value) -> anyhow::Result<Option<u32>> {
    let invalid_number = || anyhow!("Invalid numeric value for \"{}\": {:?}", label, json_value);

    match json_value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::String(value) => Ok(Some(value.parse()?)),
        serde_json::Value::Number(number) => {
            let value = number.as_u64().ok_or_else(invalid_number)?.try_into()?;
            Ok(Some(value))
        },
        _ => Err(invalid_number()),
    }
}

fn remove_nul_byte(input: &str) -> &str {
    input.trim_end_matches('\u{0000}')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_string() {
        let json = serde_json::json!("String!");
        let value = extract_string("_", &json).unwrap();
        assert_eq!(value, Some(String::from("String!")));

        let json = serde_json::json!({ "key": "String!" });
        let value = extract_string("key", &json.get("key").unwrap()).unwrap();
        assert_eq!(value, Some(String::from("String!")));

        let json = serde_json::json!({ "key": None::<String> });
        let value = extract_string("key", &json.get("key").unwrap()).unwrap();
        assert_eq!(value, None);

        let json = serde_json::json!({ "key": 13 });
        assert!(extract_string("key", &json.get("key").unwrap()).is_err());

        let json = serde_json::json!({ "key": ["String!"] });
        assert!(extract_string("key", &json.get("key").unwrap()).is_err());
    }

    #[test]
    fn test_extract_u32() {
        let json = serde_json::json!(42);
        let value = extract_u32("_", &json).unwrap();
        assert_eq!(value, Some(42));

        let json = serde_json::json!(None::<u64>);
        let value = extract_u32("_", &json).unwrap();
        assert_eq!(value, None);

        let json = serde_json::json!({ "key": "13" });
        let value = extract_u32("key", &json.get("key").unwrap()).unwrap();
        assert_eq!(value, Some(13));

        let json = serde_json::json!({ "key": "String!" });
        assert!(extract_u32("key", &json.get("key").unwrap()).is_err());

        let json = serde_json::json!({ "key": ["String!"] });
        assert!(extract_u32("key", &json.get("key").unwrap()).is_err());

        let json = serde_json::json!({ "key": u64::MAX });
        assert!(extract_u32("key", &json.get("key").unwrap()).is_err());
    }
}
