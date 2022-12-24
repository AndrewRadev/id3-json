use std::env;
use std::fs;
use std::path::{PathBuf, Path};
use std::ops::Deref;

use tempfile::TempDir;

pub struct Fixture {
    path: PathBuf,
    source: PathBuf,
    _tempdir: TempDir,
}

impl Fixture {
    pub fn blank(fixture_filename: &str) -> Self {
        let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
        let mut source = PathBuf::from(root_dir);
        source.push("tests/fixtures");
        source.push(&fixture_filename);

        let tempdir = tempfile::tempdir().unwrap();
        let mut path = PathBuf::from(&tempdir.path());
        path.push(&fixture_filename);

        Fixture { _tempdir: tempdir, source, path }
    }

    pub fn copy(fixture_filename: &str) -> Self {
        let fixture = Fixture::blank(fixture_filename);
        fs::copy(&fixture.source, &fixture.path).unwrap();
        fixture
    }
}

impl Deref for Fixture {
    type Target = Path;

    fn deref(&self) -> &Self::Target {
        self.path.deref()
    }
}
