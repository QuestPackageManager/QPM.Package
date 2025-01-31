use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::{extra::AdditionalPackageMetadata, package::PackageConfig, schema_impls::VersionReqWrapper, schema_impls::deserialize_version_req_wrapper};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "The package dependency")]
pub struct Dependency {
    pub id: String,
    #[serde(deserialize_with = "deserialize_version_req_wrapper")]
    pub version_range: VersionReqWrapper,

    // Should've been PackageDependencyModifier but oh well
    #[deprecated = "Use PackageConfig additional_data instead"]
    pub additional_data: AdditionalPackageMetadata,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "The package dependency")]
pub struct SharedDependency {
    pub dependency: Dependency,
    pub version: VersionReqWrapper,
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
