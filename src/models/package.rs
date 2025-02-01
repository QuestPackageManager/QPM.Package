use std::path::PathBuf;

use schemars::JsonSchema;
use semver::{Version, VersionReq};
use serde::{Deserialize, Deserializer, Serialize};

use super::{
    extra::{AdditionalPackageMetadata, PackageDependencyModifier},
    workspace::WorkspaceConfig,
};

use crate::models::version_req::make_version_req_schema;

/// latest version
#[inline]
fn default_ver() -> Version {
    Version::new(0, 4, 0)
}

// qpm.json
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Configuration for a package.")]
pub struct PackageConfig {
    #[serde(default = "default_ver")]
    #[schemars(description = "The version of the package configuration.")]
    pub version: Version,

    #[schemars(description = "The directory where shared files are stored.")]
    pub shared_dir: PathBuf,

    #[schemars(description = "The directory where dependencies are stored.")]
    pub dependencies_dir: PathBuf,

    #[schemars(description = "The package metadata.")]
    pub info: PackageMetadata,
    // allow workspace to be null
    #[serde(default, deserialize_with = "deserialize_null_default")]
    #[schemars(description = "The workspace configuration.")]
    pub workspace: WorkspaceConfig,

    #[schemars(description = "The dependencies of the package.")]
    pub dependencies: Vec<PackageDependency>,
}

impl Default for PackageConfig {
    fn default() -> Self {
        Self {
            version: default_ver(),
            dependencies: Default::default(),
            dependencies_dir: Default::default(),
            info: PackageMetadata {
                name: Default::default(),
                id: Default::default(),
                version: Version::new(1, 0, 0),
                url: Default::default(),
                additional_data: Default::default(),
            },
            shared_dir: Default::default(),
            workspace: Default::default(),
        }
    }
}

// qpm.json::info
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Metadata information about the package.")]
pub struct PackageMetadata {
    #[schemars(description = "The name of the package.")]
    pub name: String,

    #[schemars(description = "The unique identifier of the package.")]
    pub id: String,

    #[schemars(description = "The version of the package.")]
    pub version: Version,

    #[schemars(description = "The website for the package.")]
    pub url: Option<String>,

    #[schemars(description = "Additional metadata for the package.")]
    pub additional_data: AdditionalPackageMetadata,
}

// qpm.json::dependencies[]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "A dependency of the package.")]
pub struct PackageDependency {
    #[schemars(description = "The unique identifier of the dependency")]
    pub id: String,

    #[serde(deserialize_with = "cursed_semver_parser::deserialize")]
    #[schemars(description = "The version range of the dependency")]
    #[schemars(schema_with = "make_version_req_schema")]
    pub version_range: VersionReq,

    #[schemars(description = "Additional metadata for the dependency")]
    pub additional_data: PackageDependencyModifier,
}

fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
