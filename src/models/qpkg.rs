use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(
    description = "QPKG package. Distributes a package with all triplet binaries and their headers."
)]
pub struct QPkg {
    /// Triplet map
    pub triplets: HashMap<String, QPkgTripletInfo>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct QPkgTripletInfo {
    pub headers: Vec<PathBuf>,
    
    /// Paths to the binary files
    /// relative to the qpkg root
    pub files: Vec<PathBuf>,
}
