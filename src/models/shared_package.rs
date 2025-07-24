use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::models::triplet::{PackageTripletDependency, TripletId};

use super::package::{DependencyId, PackageConfig};

pub type SharedLockedTripletMap = HashMap<TripletId, SharedTriplet>;

// qpm.shared.json
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Configuration for a shared package.")]
pub struct SharedPackageConfig {
    /// Package name
    #[schemars(description = "Package name")]
    pub config: PackageConfig,

    pub restored_triplet: TripletId,

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
    pub restored_dependencies: HashMap<DependencyId, SharedTripletDependencyInfo>,
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
    pub restored_version: Version,
    /// Original triplet data
    pub triplet: PackageTripletDependency,
}
