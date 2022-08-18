use crate::AdditionalPackageData;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Dependency {
    pub id: String,
    /// todo: do we still use this?
    //#[serde(deserialize_with = "cursed_semver_parser::deserialize")]
    pub version_range: VersionReq,
    pub additional_data: AdditionalPackageData,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SharedDependency {
    pub dependency: Dependency,
    pub version: Version,
}
