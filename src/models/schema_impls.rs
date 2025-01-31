use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use semver::{Version, VersionReq};
use serde::{Deserialize, Deserializer, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct VersionWrapper(pub Version);
impl VersionWrapper {
    pub fn new(version: Version) -> Self {
        VersionWrapper(version)
    }
}

impl std::fmt::Display for VersionWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl JsonSchema for VersionWrapper {
    fn schema_name() -> String {
        "Version".to_string()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        String::json_schema(gen)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct VersionReqWrapper(VersionReq);
impl VersionReqWrapper {
    pub fn new(version_req: VersionReq) -> Self {
        VersionReqWrapper(version_req)
    }
}

impl std::fmt::Display for VersionReqWrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl JsonSchema for VersionReqWrapper {
    fn schema_name() -> String {
        "VersionReq".to_string()
    }

    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        String::json_schema(gen)
    }
}

pub fn deserialize_version_req_wrapper<'de, D>(deserializer: D) -> Result<VersionReqWrapper, D::Error>
where
    D: Deserializer<'de>,
{
    let version_req = VersionReq::deserialize(deserializer)?;
    Ok(VersionReqWrapper(version_req))
}
