use std::{collections::BTreeMap, path::PathBuf};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use super::schema_impls::VersionReqWrapper;

pub type WorkspaceScript = Vec<String>;

/// qpm.json::workspace
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, PartialEq, Eq, Default)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Configuration for the workspace.")]
pub struct WorkspaceConfig {
    #[serde(default)]
    #[schemars(description = "Scripts associated with the workspace.")]
    pub scripts: BTreeMap<String, WorkspaceScript>,

    /// NDK Version Range
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The NDK version range.")]
    pub ndk: Option<VersionReqWrapper>,

    #[serde(default)]
    #[schemars(description = "List of directories to search during qmod creation.")]
    pub qmod_include_dirs: Vec<PathBuf>,

    #[serde(default)]
    #[schemars(description = "List of files to include in the resulting qmod.")]
    pub qmod_include_files: Vec<PathBuf>,

    #[serde(default)]
    #[schemars(description = "Output path for the qmod.")]
    pub qmod_output: Option<PathBuf>,
}
