use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::models::package::PackageConfig;

pub const QPKG_JSON: &str = "qpm2.qpkg.json";

/// QPKG package configuration
/// Distributes a package with its binaries and headers.
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "QPKG package. Distributes a package with its binaries and headers.")]
pub struct QPkg {
    /// Package configuration
    #[schemars(description = "Package configuration")]
    pub config: PackageConfig,

    /// The directory where the headers are located
    #[schemars(description = "The directory where the headers are located")]
    pub shared_dir: PathBuf,

    /// Paths to the binary files, relative to the qpkg root
    #[schemars(description = "Paths to the binary files, relative to the qpkg root")]
    pub files: Vec<PathBuf>,
}
