use schemars::JsonSchema;
use semver::Version;
use serde::{Deserialize, Serialize};

use super::{package::PackageConfig, shared_package::SharedPackageConfig};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "The package version")]
pub struct QPackagesPackage {
    /// Package Configuration
    #[schemars(description = "Package Configuration")]
    #[serde(rename = "config")]
    pub config: PackageConfig,

    /// Checksum of the qpkg
    #[schemars(description = "Checksum of the qpkg")]
    pub qpkg_checksum: Option<String>,

    /// URL of the qpkg
    #[schemars(description = "URL of the qpkg")]
    pub qpkg_url: String,
}
