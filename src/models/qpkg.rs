use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::models::{package::PackageConfig, triplet::TripletId};

pub const QPKG_JSON: &str = "qpm2.qpkg.json";

/// QPKG package configuration
/// Distributes a package with all triplet binaries and their headers.
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(
    description = "QPKG package. Distributes a package with all triplet binaries and their headers."
)]
pub struct QPkg {
    /// Package configuration
    #[schemars(description = "Package configuration")]
    pub config: PackageConfig,

    /// The directory where the headers are located
    #[schemars(description = "The directory where the headers are located")]
    pub shared_dir: PathBuf,

    /// Triplet map
    #[schemars(description = "Triplet map")]
    pub triplets: HashMap<TripletId, QPkgTripletInfo>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct QPkgTripletInfo {
    /// Paths to the binary files
    /// relative to the qpkg root
    pub files: Vec<PathBuf>,
}
