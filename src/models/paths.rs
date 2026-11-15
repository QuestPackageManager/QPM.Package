use std::path::{Path, PathBuf};

use serde::{Serialize, Serializer, ser::Error};

// Package files are portable; keep native paths in memory and write ZIP separators.
struct PortablePath<'a>(&'a Path);

impl Serialize for PortablePath<'_> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let path = self
            .0
            .to_str()
            .ok_or_else(|| S::Error::custom("path contains invalid UTF-8"))?;
        serializer.serialize_str(&path.replace('\\', "/"))
    }
}

pub fn path<S: Serializer>(path: &Path, serializer: S) -> Result<S::Ok, S::Error> {
    PortablePath(path).serialize(serializer)
}

pub fn paths<S: Serializer>(paths: &[PathBuf], serializer: S) -> Result<S::Ok, S::Error> {
    serializer.collect_seq(paths.iter().map(|path| PortablePath(path)))
}

pub fn optional_path<S: Serializer>(
    path: &Option<PathBuf>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    path.as_deref().map(PortablePath).serialize(serializer)
}

pub fn optional_paths<S: Serializer>(
    paths: &Option<Vec<PathBuf>>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    paths
        .as_ref()
        .map(|paths| {
            paths
                .iter()
                .map(|path| PortablePath(path))
                .collect::<Vec<_>>()
        })
        .serialize(serializer)
}
