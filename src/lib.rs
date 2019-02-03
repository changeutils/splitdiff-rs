//!
//! The Splitdiff library.
//!

use std::collections::BTreeMap;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "lsdiff error {}", _0)]
    Lsdiff(lsdiff_rs::Error),
}

pub type SplitdiffResult<T> = Result<T, Error>;

pub struct PatchData(pub BTreeMap<String, Vec<Vec<String>>>);

pub struct SplitDiff {
    patch: String,
}

impl SplitDiff {
    pub fn new(patch: &str) -> Self {
        Self { patch: patch.to_owned() }
    }

    pub fn process(&self) -> SplitdiffResult<PatchData> {
        let lines: Vec<&str> = self.patch.split('\n').map(|line| line.trim()).collect();

        let mut data = BTreeMap::new();
        for entry in lsdiff_rs::process(&self.patch).map_err(Error::Lsdiff)? {
            let slice = &lines[entry.start_line..entry.start_line + entry.lines_count];
            let hunk = slice.iter().map(|v| v.to_string()).collect();
            data.entry(entry.input_path)
                .or_insert_with(Vec::new)
                .push(hunk);
        }
        Ok(PatchData(data))
    }
}
