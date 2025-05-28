use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::package::PackageConfig;

pub type SharedLockedTripletMap = HashMap<String, SharedTriplet>;

// qpm.shared.json
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Configuration for a shared package.")]
pub struct SharedPackageConfig {
    /// Package name
    #[schemars(description = "Package name")]
    pub config: PackageConfig,
    /// Triplet map
    #[schemars(description = "Triplet map")]
    pub locked_triplet: SharedLockedTripletMap,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Configuration for a shared triplet.")]
pub struct SharedTriplet {
    /// Triplet map
    #[schemars(description = "Triplet map")]
    pub restored_dependencies: HashMap<String, SharedTripletDependencyInfo>,
    // default should not appear here. All triplets should be listed
    // TODO: Include checksums here?
    // TODO: Include qpkg urls here?
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Dependency information for a shared triplet.")]
pub struct SharedTripletDependencyInfo {
    /// Version of the dependency
    #[schemars(description = "Version of the dependency.")]
    pub restored_version: Option<String>,
    /// Version range requirement
    #[schemars(description = "Version range requirement.")]
    pub version_range: String,
    /// Triplet of the dependency
    #[schemars(description = "Triplet of the dependency.")]
    pub triplet: String,
}
