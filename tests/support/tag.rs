use std::path::Path;

pub fn read_tag(path: &Path) -> id3::Tag {
    id3::Tag::read_from_path(path).unwrap()
}
