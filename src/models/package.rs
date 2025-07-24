use std::fmt::Display;
use std::path::PathBuf;

use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};

use crate::models::triplet::PackageTripletsConfig;

use super::workspace::WorkspaceConfig;

#[inline]
fn default_ver() -> Version {
    Version::new(2, 0, 0)
}

/// latest version
#[inline]
pub fn package_target_version() -> Version {
    // This will be safe since it is checked in build.rs
    Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq, Eq, Hash)]
pub struct DependencyId(pub String);

// qpm.json
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Configuration for a package.")]
pub struct PackageConfig {
    /// Package ID
    pub id: DependencyId,
    /// Package version
    pub version: Version,
    /// Directory where dependencies are restored
    pub dependencies_directory: PathBuf,
    /// Directories shared by the package
    pub shared_directories: Vec<PathBuf>,
    /// Workspace configuration
    #[serde(default)]
    pub workspace: WorkspaceConfig,
    /// Additional package metadata
    #[serde(default)]
    pub additional_data: PackageAdditionalData,
    /// Package triplet configurations
    pub triplets: PackageTripletsConfig,
    /// Config version, defaults to 2.0.0
    #[serde(default = "default_ver")]
    pub config_version: Version,

    /// Whether to generate the cmake files on restore
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Whether to generate CMake files on restore.")]
    pub cmake: Option<bool>,

    /// Whether to generate the a toolchain JSON file [CompileOptions] describing the project setup configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "Path to generate a toolchain JSON file describing the project setup configuration."
    )]
    pub toolchain_out: Option<PathBuf>,
}

#[derive(
    Serialize, Deserialize, Clone, Debug, Default, JsonSchema, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct PackageWorkspaceScripts {
    /// Scripts to run before building
    #[serde(default)]
    pub build: Vec<String>,
}

#[derive(
    Serialize, Deserialize, Clone, Debug, Default, JsonSchema, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct PackageAdditionalData {
    /// Package description
    #[serde(default)]
    pub description: String,
    /// Package author
    #[serde(default)]
    pub author: String,
    /// Package license
    #[serde(default)]
    pub license: String,
}

impl Default for PackageConfig {
    fn default() -> Self {
        Self {
            id: DependencyId::default(),
            version: default_ver(),
            dependencies_directory: "extern".into(),
            shared_directories: Vec::new(),
            workspace: Default::default(),
            additional_data: PackageAdditionalData::default(),
            triplets: PackageTripletsConfig::default(),
            config_version: default_ver(),
            cmake: None,
            toolchain_out: None,
        }
    }
}

impl Display for DependencyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
