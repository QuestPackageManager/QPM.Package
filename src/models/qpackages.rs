use schemars::JsonSchema;
use semver::Version;
use serde::{Serialize, Deserialize};

use super::{package::Package, shared_package::SharedPackage};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "The package version")]
pub struct QPackagesPackage {
    /// Package Configuration
    #[schemars(description = "Package Configuration")]
    #[serde(rename = "config")]
    pub config: Package,


    /// Checksum of the qpkg
    #[schemars(description = "Checksum of the qpkg")]
    pub qpkg_checksum: Option<String>,

    /// URL of the qpkg
    #[schemars(description = "URL of the qpkg")]
    pub qpkg_url: String,
}
