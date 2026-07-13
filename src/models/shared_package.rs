use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

use super::package::{DependencyId, EnvironmentMap, PackageConfig};

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

    /// Restored dependencies
    #[schemars(description = "Restored dependencies")]
    pub restored_dependencies: HashMap<DependencyId, SharedDependencyInfo>,

    /// Restored environment variables
    #[schemars(description = "Restored environment variables")]
    pub env: EnvironmentMap,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Dependency information for a shared package.")]
pub struct SharedDependencyInfo {
    /// Version of the dependency
    #[schemars(description = "Version of the dependency.")]
    pub restored_version: Version,

    /// Binaries restored for this dependency
    #[schemars(description = "Binaries for this dependency.")]
    pub restored_binaries: Vec<PathBuf>,

    /// Restored environment variables for the dependency
    #[schemars(description = "Restored environment variables for the dependency.")]
    pub restored_env: EnvironmentMap,
}
