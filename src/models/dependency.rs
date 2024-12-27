use semver::{Version, VersionReq};

use serde::{Deserialize, Serialize};

use super::{extra::AdditionalPackageMetadata, package::PackageConfig};

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub id: String,
    #[serde(deserialize_with = "cursed_semver_parser::deserialize")]
    pub version_range: VersionReq,

    // Should've been PackageDependencyModifier but oh well
    #[deprecated = "Use PackageConfig additional_data instead"]
    pub additional_data: AdditionalPackageMetadata,
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SharedDependency {
    pub dependency: Dependency,
    pub version: Version,
}

/// qpm.shared.json
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct SharedPackageConfig {
    /// The package config that is stored in qpm.json, copied
    pub config: PackageConfig,
    /// The dependencies as given by self.config.resolve()
    pub restored_dependencies: Vec<SharedDependency>,
}
