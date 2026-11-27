use std::collections::BTreeMap;
use std::path::PathBuf;

use schemars::JsonSchema;
use semver::VersionReq;
use serde::{Deserialize, Serialize};

use super::version_req::make_version_req_schema;

pub type WorkspaceScript = Vec<String>;
pub type EnvironmentMap = std::collections::HashMap<String, String>;

/// qpm.json::workspace
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq, Default)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Configuration for the workspace.")]
pub struct WorkspaceConfig {
    #[serde(default)]
    #[schemars(description = "Scripts associated with the workspace.")]
    pub scripts: BTreeMap<String, WorkspaceScript>,

    /// Environment variables for the workspace
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Environment variables for the workspace.")]
    pub env: Option<EnvironmentMap>,

    /// NDK version range requirement
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "NDK version range requirement.")]
    #[schemars(schema_with = "make_version_req_schema")]
    pub ndk: Option<VersionReq>,

    /// Paths to the output binaries, relative to the directory containing qpm2.json.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "super::paths::optional_paths")]
    #[schemars(
        description = "Paths to the output binaries, relative to the directory containing qpm2.json."
    )]
    pub out_binaries: Option<Vec<PathBuf>>,

    /// Optional QPKG header selection and installed path prefix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header_export: Option<HeaderExportConfig>,

    /// Path to generate a toolchain JSON file describing the project setup configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "super::paths::optional_path")]
    #[schemars(description = "Path to generate a toolchain JSON file describing the project setup configuration.")]
    pub toolchain_out: Option<PathBuf>,

    /// Whether to generate the cmake files on restore
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Whether to generate CMake files on restore.")]
    pub cmake: Option<bool>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "camelCase")]
pub struct HeaderExportConfig {
    /// Prefix under the dependency's restored include directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "super::paths::optional_path")]
    #[schemars(description = "Optional prefix inside the restored include directory.")]
    pub prefix: Option<PathBuf>,

    /// Files and directory trees to export, relative to sharedDirectory.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(serialize_with = "super::paths::optional_paths")]
    #[schemars(
        description = "Files and directory trees relative to sharedDirectory. Defaults to the entire shared directory."
    )]
    pub paths: Option<Vec<PathBuf>>,
}
