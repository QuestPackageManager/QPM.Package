use schemars::JsonSchema;
use serde::{Serialize, Deserialize};
use crate::models::schema_impls::VersionWrapper;

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Hash, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "The package version")]
pub struct PackageVersion {
    #[schemars(description = "The unique identifier of the package.")]
    pub id: String,

    #[schemars(description = "The version of the package.")]
    pub version: VersionWrapper,
}
