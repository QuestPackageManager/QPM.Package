use std::path::PathBuf;

use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

use super::{
    extra::{AdditionalPackageMetadata, PackageDependencyModifier},
    workspace::WorkspaceConfig,
};

fn default_ver() -> Version {
    // This will be true since it is checked in build.rs
    Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}

// qpm.json
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct PackageConfig {
    #[serde(default = "default_ver")]
    pub version: Version,
    pub shared_dir: PathBuf,
    pub dependencies_dir: PathBuf,
    pub info: PackageMetadata,
    #[serde(default)]
    pub workspace: WorkspaceConfig,
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
