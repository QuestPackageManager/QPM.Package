use super::version_req::make_version_req_schema;
use schemars::JsonSchema;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use crate::models::triplet::{TripletEnvironmentMap, TripletId};

use super::package::{DependencyId, PackageConfig};

pub type SharedLockedTripletMap = HashMap<TripletId, SharedTriplet>;

pub const QPM_SHARED_JSON: &str = "qpm2.shared.json";

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

    #[schemars(description = "Environment variables for the triplet.")]
    pub env: TripletEnvironmentMap,

    /// Output binaries for this triplet
    #[schemars(description = "Output binaries for this triplet.")]
    pub out_binaries: Vec<PathBuf>,
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

    // #[schemars(schema_with = "make_version_req_schema")]
    // pub version_range: VersionReq,

    /// Original triplet data
    /// This is the triplet ID of the original triplet that this dependency was restored from.
    #[schemars(description = "Restored triplet ID of the dependency.")]
    pub restored_triplet: TripletId,

    /// Binaries restored for this triplet
    #[schemars(description = "Binaries for this triplet.")]
    pub restored_binaries: Vec<PathBuf>,

    /// Restored environment variables for the triplet
    #[schemars(description = "Restored environment variables for the triplet.")]
    pub restored_env: TripletEnvironmentMap,
}
