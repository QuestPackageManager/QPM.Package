use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "QPKG package. Distributes a package with all triplet binaries and their headers.")]
pub struct QPkg {
    /// Triplet map
    pub triplets: HashMap<String, QPkgTripletInfo>,

    pub header_file: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
pub struct QPkgTripletInfo {
    /// Paths to the binary files
    pub files: Vec<String>,
}
