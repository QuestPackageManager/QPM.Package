use std::{collections::HashMap, fmt::Display};

use schemars::JsonSchema;
use semver::VersionReq;
use serde::{Deserialize, Serialize};

use super::version_req::make_version_req_schema;

use crate::models::{extra::PackageTripletCompileOptions, package::DependencyId};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq, Eq, Hash)]
pub struct TripletId(pub String);

/// Dependency ID -> Dependency
pub type TripletDependencyMap = HashMap<DependencyId, PackageTripletDependency>;

/// ENV -> VALUE
pub type TripletEnvironmentMap = HashMap<String, String>;

/// Package triplet configuration
#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Configuration for a package's triplets map")]
pub struct PackageTripletsConfig {
    /// Default configuration for all triplets. All triplets will inherit from this.
    #[serde(default)]
    pub default: PackageTriplet,
    /// Configuration for specific triplets
    #[serde(flatten)]
    pub specific_triplets: HashMap<TripletId, PackageTriplet>,
}

impl PackageTripletsConfig {
    /// Retrieves the settings for a specific triplet, merging with default settings.
    ///
    /// This function looks up settings for the specified triplet and combines them with
    /// the default settings to create a complete configuration. The merging strategy is:
    ///
    /// - Dependencies: Both specific and default dependencies are included
    /// - Environment variables: Both specific and default variables are included, with specific ones taking precedence
    /// - Compile options: Merged with default options if present
    /// - QMod URL: Uses the specific URL if available, otherwise falls back to the default
    ///
    /// # Parameters
    /// * `triplet` - The triplet identifier to look up settings for
    ///
    /// # Returns
    /// * `Some(PackageTripletSettings)` if the triplet exists in the configuration
    /// * `None` if the triplet is not found in the specific_triplets map
    pub fn get_triplet_settings(&self, triplet: &TripletId) -> Option<PackageTriplet> {
        let found = self.specific_triplets.get(triplet)?;

        let default = &self.default;
        let mut dependencies = found.dependencies.clone();
        dependencies.extend(default.dependencies.clone());

        let mut dev_dependencies = found.dependencies.clone();
        dev_dependencies.extend(default.dev_dependencies.clone());

        let mut env = found.env.clone();
        env.extend(default.env.clone());

        let compile_options = found
            .compile_options
            .clone()
            .map(|a| a.merge(self.default.compile_options.clone().unwrap_or_default()));

        Some(PackageTriplet {
            dependencies,
            dev_dependencies,
            env,
            compile_options,
            qmod_url: found.qmod_url.clone().or(default.qmod_url.clone()),
            qmod_id: found.qmod_id.clone().or(default.qmod_id.clone()),
        })
    }
}

/// Triplet
#[derive(Serialize, Deserialize, Clone, Debug, Default, JsonSchema, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Configuration for a package triplet.")]
pub struct PackageTriplet {
    /// Dependencies for this triplet
    #[serde(default)]
    pub dependencies: TripletDependencyMap,

    /// Dependencies for this triplet
    #[serde(default)]
    pub dev_dependencies: TripletDependencyMap,

    // TODO: use PackageTripletSettings
    /// Environment variables for this triplet.
    #[serde(default)]
    pub env: TripletEnvironmentMap,

    /// Additional Compile options to be used with this package
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Additional compile options for the package.")]
    pub compile_options: Option<PackageTripletCompileOptions>,

    /// QMod URL for this triplet
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "QMod URL for this triplet.")]
    pub qmod_url: Option<String>,

    /// QMod URL for this triplet
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "QMod ID for this triplet.")]
    pub qmod_id: Option<String>,
}

impl PackageTriplet {
    pub fn get_dependency(&self, dep_id: &DependencyId) -> Option<&PackageTripletDependency> {
        self.dependencies
            .get(dep_id)
            .or_else(|| self.dev_dependencies.get(dep_id))
    }

    pub fn get_dependencies_combined(
        &self,
    ) -> impl Iterator<Item = (&DependencyId, &PackageTripletDependency)> {
        self.dependencies.iter().chain(self.dev_dependencies.iter())
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq, Eq)]
#[allow(non_snake_case)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Dependency information for a package triplet.")]
pub struct PackageTripletDependency {
    /// Version range requirement
    #[serde(rename = "versionRange")]
    #[schemars(schema_with = "make_version_req_schema")]
    pub version_range: VersionReq,
    /// Target triplet
    pub triplet: TripletId,
    /// Whether to include this dependency in the qmod
    #[serde(default)]
    pub qmod_export: bool,

    /// QMod URL for this triplet
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "QMod required for this triplet.")]
    pub qmod_required: Option<bool>,
}

impl Display for TripletId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
