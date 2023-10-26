use std::collections::{BTreeMap};

use semver::{VersionReq};
use serde::{Serialize, Deserialize};

pub type WorkspaceScript = Vec<String>;


/// qpm.json::workspace
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceConfig {
    #[serde(default)]
    pub scripts: BTreeMap<String, WorkspaceScript>,

    /// NDK Version Range
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ndk: Option<VersionReq>,


}