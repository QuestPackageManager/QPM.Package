use std::collections::HashMap;
use std::fmt::Display;
use std::path::PathBuf;

use schemars::JsonSchema;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::models::extra::PackageCompileOptions;

use super::version_req::make_version_req_schema;
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

pub const QPM_JSON: &str = "qpm2.json";

/// Represents the game id for a QMOD package.
pub const QPM_ENV_GAME_ID: &str = "QMOD_GAME_ID";
/// Represents the game version for a QMOD package.
pub const QPM_ENV_GAME_VERSION: &str = "QMOD_GAME_VERSION";

#[derive(
    Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct DependencyId(pub String);

/// Dependency ID -> Dependency
pub type DependencyMap = HashMap<DependencyId, PackageDependency>;

/// ENV -> VALUE
pub type EnvironmentMap = HashMap<String, String>;

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
    pub shared_directory: PathBuf,
    /// Workspace configuration
    #[serde(default)]
    pub workspace: WorkspaceConfig,
    /// Additional package metadata
    #[serde(default)]
    pub additional_data: PackageAdditionalData,

    /// Dependencies for this package
    #[serde(default)]
    pub dependencies: DependencyMap,

    /// Dev dependencies for this package
    #[serde(default)]
    pub dev_dependencies: DependencyMap,

    /// Environment variables for this package.
    #[serde(default)]
    pub env: EnvironmentMap,

    /// Additional Compile options to be used with this package
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Additional compile options for the package.")]
    pub compile_options: Option<PackageCompileOptions>,

    /// QMod URL for this package
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "QMod URL for this package.")]
    pub qmod_url: Option<String>,

    /// QMod ID for this package
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "QMod ID for this package.")]
    pub qmod_id: Option<String>,

    #[serde(default)]
    #[schemars(description = "List of directories to search during qmod creation.")]
    pub qmod_include_dirs: Vec<PathBuf>,

    #[serde(default)]
    #[schemars(description = "List of files to include in the resulting qmod.")]
    pub qmod_include_files: Vec<PathBuf>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Output path for the qmod.")]
    pub qmod_output: Option<PathBuf>,

    /// QMod template path for this package e.g mod.template.json
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "QMod template for this package.")]
    pub qmod_template: Option<PathBuf>,

    /// NDK Version Range
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The NDK version range.")]
    #[schemars(schema_with = "make_version_req_schema")]
    pub ndk: Option<VersionReq>,

    /// Output binaries for this package
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Output binaries for this package.")]
    pub out_binaries: Option<Vec<PathBuf>>,

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

impl PackageConfig {
    pub fn get_dependency(&self, dep_id: &DependencyId) -> Option<&PackageDependency> {
        self.dependencies
            .get(dep_id)
            .or_else(|| self.dev_dependencies.get(dep_id))
    }

    pub fn get_dependencies_combined(
        &self,
    ) -> impl Iterator<Item = (&DependencyId, &PackageDependency)> {
        self.dependencies
            .iter()
            .chain(self.dev_dependencies.iter())
    }
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

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Dependency information for a package.")]
pub struct PackageDependency {
    /// Version range requirement
    #[serde(rename = "versionRange")]
    #[schemars(schema_with = "make_version_req_schema")]
    pub version_range: VersionReq,

    /// Whether to include this dependency in the qmod
    #[serde(default)]
    pub qmod_export: bool,

    /// Whether this is required/optional in the qmod
    /// QMod required field for this dependency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "QMod required field for this dependency.")]
    pub qmod_required: Option<bool>,
}

impl Default for PackageConfig {
    fn default() -> Self {
        Self {
            id: DependencyId::default(),
            version: default_ver(),
            dependencies_directory: "extern".into(),
            shared_directory: "shared".into(),
            workspace: Default::default(),
            additional_data: PackageAdditionalData::default(),
            dependencies: Default::default(),
            dev_dependencies: Default::default(),
            env: Default::default(),
            compile_options: None,
            qmod_url: None,
            qmod_id: None,
            qmod_include_dirs: Default::default(),
            qmod_include_files: Default::default(),
            qmod_output: None,
            qmod_template: None,
            ndk: None,
            out_binaries: None,
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
