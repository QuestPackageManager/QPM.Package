use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use serde::{Deserialize, Serialize};

use super::{extra::AdditionalPackageMetadata, package::PackageConfig, schema_impls::VersionReqWrapper, schema_impls::deserialize_version_req_wrapper};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "A dependency of the package.")]
pub struct Dependency {
    pub id: String,
    #[serde(deserialize_with = "deserialize_version_req_wrapper")]
    #[schemars(description = "The version range of the dependency")]
    pub version_range: VersionReqWrapper,

    // Should've been PackageDependencyModifier but oh well
    #[deprecated = "Use PackageConfig additional_data instead"]
    #[schemars(description = "Additional metadata for the dependency. Deprecated, use packageConfig.additionalData instead.")]
    pub additional_data: AdditionalPackageMetadata,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "A resolved dependency of the package.")]
pub struct SharedDependency {
    #[schemars(description = "The resolved dependency")]
    pub dependency: Dependency,

    #[schemars(description = "The resolved version of the dependency")]
    pub version: VersionReqWrapper,
}

/// qpm.shared.json
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Shared package configuration.")]
pub struct SharedPackageConfig {
    /// The package config that is stored in qpm.json, copied
    #[schemars(description = "A copy of the package configuration stored in qpm.json for convenience.")]
    pub config: PackageConfig,
    /// The dependencies as given by self.config.resolve()
    #[schemars(description = "The resolved dependencies of the package.")]
    pub restored_dependencies: Vec<SharedDependency>,
}
