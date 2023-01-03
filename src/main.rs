use std::process::ExitCode;

use id3_json::input;
use id3_json::json;

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            print_json_error(e);
            ExitCode::FAILURE
        },
    }
}

fn run() -> anyhow::Result<()> {
    let args = input::parse_args(std::env::args_os())?;

    let mut tag = match id3::Tag::read_from_path(&args.filename) {
        Ok(t) => t,
        Err(id3::Error { kind: id3::ErrorKind::NoTag, .. }) => {
            println!("{{\"data\":{{}}}}");
            return Ok(());
        },
        Err(e) => return Err(e.into()),
    };

    if args.write {
        let input = serde_json::from_reader(std::io::stdin())?;

        json::write_to_tag(input, &mut tag)?;

        let tag_version = args.tag_version.unwrap_or_else(|| tag.version());
        tag.write_to_path(&args.filename, tag_version)?;
    }

    if args.read {
        let tag_json = json::read_from_tag(&tag);
        serde_json::to_writer(std::io::stdout(), &tag_json)?;
    }

    Ok(())
}

fn print_json_error(e: anyhow::Error) {
    let error_json = serde_json::json!({ "error": format!("{}", e) });
    // Unwrap: If writing to stdout fails, we might as well panic at this point
    serde_json::to_writer(std::io::stdout(), &error_json).unwrap();
}
