use std::{collections::BTreeMap, path::PathBuf};

use schemars::JsonSchema;
use semver::VersionReq;
use serde::{Deserialize, Serialize};

use crate::models::version_req::make_version_req_schema;

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

}
