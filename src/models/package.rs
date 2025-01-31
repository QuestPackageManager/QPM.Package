use std::path::PathBuf;

use semver::Version;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize, Deserializer};

use super::{
    extra::{AdditionalPackageMetadata, PackageDependencyModifier}, schema_impls::{VersionReqWrapper, VersionWrapper, deserialize_version_req_wrapper}, workspace::WorkspaceConfig
};

#[inline]
fn default_ver() -> VersionWrapper {
    VersionWrapper(Version::new(0,4,0))
}

/// latest version
#[inline]
pub fn package_target_version() -> Version {
    // This will be safe since it is checked in build.rs
    Version::parse(env!("CARGO_PKG_VERSION")).unwrap()
}

// qpm.json
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Configuration for a package.")]
pub struct PackageConfig {
    #[serde(default = "default_ver")]
    #[schemars(description = "The version of the package configuration.")]
    pub version: VersionWrapper,

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
                version: VersionWrapper(Version::new(1, 0, 0)),
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
pub struct PackageMetadata {
    #[schemars(description = "The name of the package.")]
    pub name: String,

    #[schemars(description = "The unique identifier of the package.")]
    pub id: String,

    #[schemars(description = "The version of the package.")]
    pub version: VersionWrapper,

    #[schemars(description = "The website for the package.")]
    pub url: Option<String>,

    #[schemars(description = "Additional metadata for the package.")]
    pub additional_data: AdditionalPackageMetadata,
}

// qpm.json::dependencies[]
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PackageDependency {
    pub id: String,
    #[serde(deserialize_with = "deserialize_version_req_wrapper")]
    pub version_range: VersionReqWrapper,
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
