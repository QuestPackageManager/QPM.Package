use schemars::{gen::SchemaGenerator, schema::Schema, JsonSchema};
use semver::{Version, VersionReq};
use serde::{Deserialize, Deserializer, Serialize};

/// Wrapper for the `Version` struct from the `semver` crate.
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct VersionWrapper(pub Version);

impl VersionWrapper {
    /// Creates a new `VersionWrapper`.
    ///
    /// # Arguments
    ///
    /// * `version` - A `Version` instance to wrap.
    pub fn new(version: Version) -> Self {
        VersionWrapper(version)
    }
}

impl std::fmt::Display for VersionWrapper {
    /// Formats the `VersionWrapper` for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl JsonSchema for VersionWrapper {
    /// Returns the schema name for `VersionWrapper`.
    fn schema_name() -> String {
        "Version".to_string()
    }

    /// Generates the JSON schema for `VersionWrapper`.
    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        String::json_schema(gen)
    }
}

/// Wrapper for the `VersionReq` struct from the `semver` crate.
#[derive(Serialize, Deserialize, Clone, Debug, Hash, PartialEq, Eq)]
pub struct VersionReqWrapper(VersionReq);

impl VersionReqWrapper {
    /// Creates a new `VersionReqWrapper`.
    ///
    /// # Arguments
    ///
    /// * `version_req` - A `VersionReq` instance to wrap.
    pub fn new(version_req: VersionReq) -> Self {
        VersionReqWrapper(version_req)
    }
}

impl std::fmt::Display for VersionReqWrapper {
    /// Formats the `VersionReqWrapper` for display.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl JsonSchema for VersionReqWrapper {
    /// Returns the schema name for `VersionReqWrapper`.
    fn schema_name() -> String {
        "VersionReq".to_string()
    }

    /// Generates the JSON schema for `VersionReqWrapper`.
    fn json_schema(gen: &mut SchemaGenerator) -> Schema {
        String::json_schema(gen)
    }
}

/// Deserializes a `VersionReqWrapper`.
///
/// # Arguments
///
/// * `deserializer` - The deserializer to use.
///
/// # Returns
///
/// A `Result` containing the deserialized `VersionReqWrapper` or an error.
pub fn deserialize_version_req_wrapper<'de, D>(deserializer: D) -> Result<VersionReqWrapper, D::Error>
where
    D: Deserializer<'de>,
{
    let version_req = VersionReq::deserialize(deserializer)?;
    Ok(VersionReqWrapper(version_req))
}
