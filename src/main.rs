mod input;
mod json;

fn main() -> anyhow::Result<()> {
    let args = input::parse_args()?;

    let mut tag = match id3::Tag::read_from_path(&args.filename) {
        Ok(t) => t,
        Err(id3::Error { kind: id3::ErrorKind::NoTag, .. }) => {
            println!("{{}}");
            return Ok(());
        },
        Err(e) => return Err(e.into()),
    };

    if args.write {
        let input = serde_json::from_reader(std::io::stdin())?;

        json::write_to_tag(input, &mut tag)?;
        tag.write_to_path(&args.filename, id3::Version::Id3v23)?;
    }

    if args.read {
        let tag_json = json::read_from_tag(&tag);
        serde_json::to_writer(std::io::stdout(), &tag_json)?;
    }

    Ok(())
}
