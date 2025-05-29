use std::collections::HashMap;
use std::path::PathBuf;

use schemars::JsonSchema;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::extensions::serde_utils::deserialize_null_default;
use crate::models::version_req::make_version_req_schema;

use super::extra::CompileOptions;

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

// qpm.json
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Configuration for a package.")]
pub struct PackageConfig {
    /// Package ID
    pub id: String,
    /// Package version
    pub version: Version,
    /// Directory where dependencies are restored
    pub dependencies_directory: String,
    /// Directories shared by the package
    pub shared_directories: Vec<String>,
    /// Workspace configuration
    #[serde(default)]
    pub workspace: PackageWorkspace,
    /// Additional package metadata
    #[serde(default)]
    pub additional_data: PackageAdditionalData,
    /// Package triplet configurations
    pub triplet: PackageTripletsConfig,
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

impl Default for PackageConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            version: default_ver(),
            dependencies_directory: "extern".to_string(),
            shared_directories: Vec::new(),
            workspace: PackageWorkspace::default(),
            additional_data: PackageAdditionalData::default(),
            triplet: PackageTripletsConfig::default(),
            config_version: default_ver(),
            cmake: None,
            toolchain_out: None,
        }
    }
}

#[derive(
    Serialize, Deserialize, Clone, Debug, Default, JsonSchema, PartialEq, Eq, PartialOrd, Ord,
)]
pub struct PackageWorkspace {
    /// Scripts to run at different stages
    #[serde(default)]
    pub scripts: PackageWorkspaceScripts,
    /// Directories to search for qmod files
    #[serde(default)]
    pub qmod_search_dirs: Vec<String>,
    /// Files to include in the qmod
    #[serde(default)]
    pub qmod_include_files: Vec<String>,
    /// Output directory for the qmod
    #[serde(default, deserialize_with = "deserialize_null_default")]
    pub qmod_output: Option<String>,
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

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq, Eq, Hash)]
pub struct TripletId(String);

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq, Eq, Hash)]
pub struct DependencyId(String);

// Dependency ID -> Dependency
pub type TripletDependencyMap = HashMap<DependencyId, PackageTripletDependency>;
// ENV -> VALUE
pub type TripletEnvironmentMap = HashMap<String, String>;

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq, Eq)]
pub struct PackageTripletsConfig {
    /// Default configuration for all triplets. All triplets will inherit from this.
    pub default: PackageTripletSettings,
    /// Configuration for specific triplets
    #[serde(flatten)]
    pub specific_triplets: HashMap<TripletId, PackageTripletSettings>,
}

/// Triplet settings for a package
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema, PartialEq, Eq)]
pub struct PackageTripletSettings {
    /// Dependencies for this triplet
    #[serde(default)]
    pub dependencies: TripletDependencyMap,

    /// Environment variables for this triplet.
    #[serde(default)]
    pub env: TripletEnvironmentMap,

    /// Additional Compile options to be used with this package
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Additional compile options for the package.")]
    pub compile_options: Option<CompileOptions>,

    /// QMod URL for this triplet
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "QMod URL for this triplet.")]
    pub qmod_url: Option<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq, Eq)]
pub struct PackageTripletDependency {
    /// Version range requirement
    #[serde(rename = "versionRange")]
    #[schemars(schema_with = "make_version_req_schema")]
    pub version_range: VersionReq,
    /// Target triplet
    pub triplet: TripletId,
    /// Whether to export this dependency to consumers
    #[serde(default)]
    pub export: bool,
    /// Whether to include this dependency in the qmod
    #[serde(default)]
    pub qmod_export: bool,
}
