use std::cmp;
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::*;

#[derive(Debug)]
pub(crate) struct DirsEntry {
    pub(crate) name: String,
    pub(crate) value: String,
}

impl DirsEntry {
    fn from_line(line: &str) -> Result<Self> {
        let mut parts = line.split('=');
        let name = Self::to_string(parts.next())?;
        let value = Self::to_string(parts.next())?
            .trim_start_matches('"')
            .trim_end_matches('"')
            .to_string();
        Ok(Self { name, value })
    }

    fn to_string(option: Option<&str>) -> Result<String> {
        match option {
            Some(v) => Ok(v.to_string()),
            None => Err(Error::NotEnoughParts)?,
        }
    }
}

#[derive(Debug)]
pub(crate) struct DirsFile {
    path: PathBuf,
}

impl DirsFile {
    pub(crate) fn new(path: &Path) -> Self {
        let path = PathBuf::from(path);
        Self { path }
    }

    pub(crate) fn entries(&self) -> Result<Vec<DirsEntry>> {
        let content = match fs::read_to_string(&self.path) {
            Ok(v) => v,
            Err(e) => return Err(Error::StdIoError(e)),
        };

        let mut entries = vec![];
        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() || line.starts_with("#") {
                continue;
            }
            entries.push(DirsEntry::from_line(line).map_err(|err| {
                Error::InvalidLine(line[..cmp::min(line.len(), 8)].to_string(), err.to_string())
            })?);
        }

        Ok(entries)
    }
}
