use anyhow::anyhow;
use id3::TagLike;

pub fn read_from_tag(tag: &id3::Tag) -> serde_json::Value {
    // There could be many comments, but in my music library, it seems like it's common to just
    // have one with a "description" set to an empty string. So let's have a single "comment" field
    // that reads and writes there.
    let comment = tag.comments().find(|c| c.description == "").map(|c| c.text.clone());

    serde_json::json!({
        "data": {
            "title": tag.title(),
            "artist": tag.artist(),
            "album": tag.album(),
            "track": tag.track(),
            "genre": tag.genre(),
            "comment": comment,
        },
    })
}

pub fn write_to_tag(
    json_map: serde_json::Map<String, serde_json::Value>,
    tag: &mut id3::Tag,
) -> anyhow::Result<()> {
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
                    position(|c| c.content().comment().unwrap().description == "");
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
                    (None, None) => return Ok(()),
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


pub fn extract_string(label: &str, json_value: &serde_json::Value) -> anyhow::Result<Option<String>> {
    match json_value {
        serde_json::Value::Null          => Ok(None),
        serde_json::Value::String(value) => Ok(Some(value.clone())),
        _ => Err(anyhow!("Invalid string value for \"{}\": {:?}", label, json_value)),
    }
}

pub fn extract_u32(label: &str, json_value: &serde_json::Value) -> anyhow::Result<Option<u32>> {
    let invalid_number = anyhow!("Invalid numeric value for \"{}\": {:?}", label, json_value);

    match json_value {
        serde_json::Value::Null => Ok(None),
        serde_json::Value::Number(number) => {
            let value = number.as_u64().ok_or_else(|| invalid_number)?.try_into()?;
            Ok(Some(value))
        },
        _ => Err(anyhow!("Invalid numeric value for \"{}\": {:?}", label, json_value)),
    }
}
