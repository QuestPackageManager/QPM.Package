use std::path::PathBuf;

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use super::{
    extra::{AdditionalPackageMetadata, PackageDependencyModifier},
    workspace::WorkspaceConfig,
};

// qpm.json
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct PackageConfig {
    pub shared_dir: PathBuf,
    pub dependencies_dir: PathBuf,
    pub info: PackageMetadata,
    #[serde(default)]
    pub workspace: Option<WorkspaceConfig>,
    pub dependencies: Vec<PackageDependency>,
}

// qpm.json::info
#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PackageMetadata {
    pub name: String,
    pub id: String,
    pub version: Version,
    pub url: Option<String>,
    pub additional_data: AdditionalPackageMetadata,
}

// qpm.json::dependencies[]
#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PackageDependency {
    pub id: String,
    #[serde(deserialize_with = "cursed_semver_parser::deserialize")]
    pub version_range: VersionReq,
    pub additional_data: PackageDependencyModifier,
}
