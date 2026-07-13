use std::fmt::Display;
use std::path::PathBuf;

use schemars::JsonSchema;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use crate::models::extra::PackageCompileOptions;

use super::version_req::make_version_req_schema;
pub use super::workspace::{WorkspaceConfig, EnvironmentMap};

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

/// QMod configuration
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq, Default)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "QMod package configuration.")]
pub struct QmodConfig {
    /// The output name of the QMOD file
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The output name of the QMOD, default to something based on the package ID.")]
    pub output: Option<PathBuf>,

    /// The template path for the mod.json
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The template path for the mod.json, default to mod.template.json.")]
    pub template: Option<PathBuf>,

    /// Directories to search during qmod creation
    #[serde(default)]
    #[schemars(description = "Directories to search during qmod creation.")]
    pub search_dirs: Vec<PathBuf>,

    /// Files to include in the resulting qmod
    #[serde(default)]
    #[schemars(description = "Files to include in the resulting qmod.")]
    pub include_files: Vec<PathBuf>,

    /// Download URL used in the generated mod.json
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Download URL used in the generated mod.json.")]
    pub download_url: Option<String>,

    /// QMod ID for this package
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "QMod ID for this package.")]
    pub id: Option<String>,
}

impl QmodConfig {
    fn is_empty(&self) -> bool {
        self.output.is_none()
            && self.template.is_none()
            && self.search_dirs.is_empty()
            && self.include_files.is_empty()
            && self.download_url.is_none()
            && self.id.is_none()
    }
}

#[derive(
    Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
pub struct DependencyId(pub String);

/// Dependency ID -> Dependency
pub type DependencyMap = std::collections::HashMap<DependencyId, PackageDependency>;

// qpm.json
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Configuration for a package.")]
pub struct PackageConfig {
    /// Config version, defaults to 2.0.0
    #[serde(default = "default_ver")]
    pub config_version: Version,

    /// Package ID
    pub id: DependencyId,

    /// Package version
    pub version: Version,

    /// Additional package metadata
    #[serde(default)]
    pub additional_data: PackageAdditionalData,

    /// Workspace configuration
    #[serde(default)]
    pub workspace: WorkspaceConfig,

    /// QMod configuration
    #[serde(default, skip_serializing_if = "QmodConfig::is_empty")]
    pub qmod: QmodConfig,

    /// Dependencies for this package
    #[serde(default)]
    pub dependencies: DependencyMap,

    /// Dev dependencies for this package
    #[serde(default)]
    pub dev_dependencies: DependencyMap,

    /// Directory where dependencies are restored
    pub dependencies_directory: PathBuf,

    /// Directories shared by the package
    pub shared_directory: PathBuf,

    /// Additional Compile options to be used with this package
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Additional compile options for the package.")]
    pub compile_options: Option<PackageCompileOptions>,
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
    /// General URL for the mod
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "General URL for the mod.")]
    pub url: Option<String>,
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
            config_version: default_ver(),
            id: DependencyId::default(),
            version: default_ver(),
            additional_data: PackageAdditionalData::default(),
            workspace: WorkspaceConfig::default(),
            qmod: QmodConfig::default(),
            dependencies: DependencyMap::default(),
            dev_dependencies: DependencyMap::default(),
            dependencies_directory: "extern".into(),
            shared_directory: "shared".into(),
            compile_options: None,
        }
    }
}

impl Display for DependencyId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
