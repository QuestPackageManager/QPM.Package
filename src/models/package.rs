use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;

use schemars::JsonSchema;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::models::version_req::make_version_req_schema;

use super::extra::CompileOptions;
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
pub struct TripletId(pub String);

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq, Eq, Hash)]
pub struct DependencyId(pub String);

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

impl PackageTripletsConfig {
    /// Retrieves the settings for a specific triplet, merging with default settings.
    ///
    /// This function looks up settings for the specified triplet and combines them with
    /// the default settings to create a complete configuration. The merging strategy is:
    ///
    /// - Dependencies: Both specific and default dependencies are included
    /// - Environment variables: Both specific and default variables are included, with specific ones taking precedence
    /// - Compile options: Merged with default options if present
    /// - QMod URL: Uses the specific URL if available, otherwise falls back to the default
    ///
    /// # Parameters
    /// * `triplet` - The triplet identifier to look up settings for
    ///
    /// # Returns
    /// * `Some(PackageTripletSettings)` if the triplet exists in the configuration
    /// * `None` if the triplet is not found in the specific_triplets map
    pub fn get_triplet_settings(&self, triplet: &TripletId) -> Option<PackageTripletSettings> {
        let found = self.specific_triplets.get(triplet)?;

        let default = &self.default;
        let mut dependencies = found.dependencies.clone();
        dependencies.extend(default.dependencies.clone());

        let mut env = found.env.clone();
        env.extend(default.env.clone());

        let compile_options = found
            .compile_options
            .clone()
            .map(|a| a.merge(self.default.compile_options.clone().unwrap_or_default()));

        Some(PackageTripletSettings {
            dependencies,
            env,
            compile_options,
            qmod_url: found.qmod_url.clone().or(default.qmod_url.clone()),
        })
    }
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

impl CompileOptions {
    pub fn merge(self, other: CompileOptions) -> Self {
        Self {
            c_flags: self.c_flags.or(other.c_flags),
            cpp_features: self.cpp_features.or(other.cpp_features),
            cpp_flags: self.cpp_flags.or(other.cpp_flags),
            include_paths: self.include_paths.or(other.include_paths),
            system_includes: self.system_includes.or(other.system_includes),
        }
    }
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

impl Default for PackageConfig {
    fn default() -> Self {
        Self {
            id: DependencyId::default(),
            version: default_ver(),
            dependencies_directory: "extern".into(),
            shared_directories: Vec::new(),
            workspace: Default::default(),
            additional_data: PackageAdditionalData::default(),
            triplet: PackageTripletsConfig::default(),
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

impl Display for TripletId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
