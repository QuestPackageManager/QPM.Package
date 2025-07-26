use std::{borrow::Cow, collections::HashMap, fmt::Display, path::PathBuf};

use schemars::JsonSchema;
use semver::VersionReq;
use serde::{Deserialize, Serialize};

use super::version_req::make_version_req_schema;

use crate::models::{extra::PackageTripletCompileOptions, package::DependencyId};

#[derive(Serialize, Deserialize, Clone, Debug, JsonSchema, Default, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TripletId(pub String);

/// Dependency ID -> Dependency
pub type TripletDependencyMap = HashMap<DependencyId, PackageTripletDependency>;

/// ENV -> VALUE
pub type TripletEnvironmentMap = HashMap<String, String>;

/// Represents the game id for a QMOD package.
pub const QPM_ENV_GAME_ID: &str = "QMOD_GAME_ID";
/// Represents the game version for a QMOD package.
pub const QPM_ENV_GAME_VERSION: &str = "QMOD_GAME_VERSION";

pub fn default_triplet_id() -> TripletId {
    TripletId("default".to_owned())
}

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
        if triplet == &default_triplet_id() {
            return Some(self.default.clone());
        }

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
            out_binaries: found.out_binaries.clone().or(default.out_binaries.clone()),
            qmod_url: found.qmod_url.clone().or(default.qmod_url.clone()),
            qmod_id: found.qmod_id.clone().or(default.qmod_id.clone()),
        })
    }

    /// Iterates over all triplets, including the default one.
    pub fn iter_triplets(&self) -> impl Iterator<Item = (TripletId, Cow<PackageTriplet>)> {
        let other = self.specific_triplets.keys().map(|k| {
            let package_triplet = self.get_triplet_settings(k).unwrap();

            (k.clone(), Cow::Owned(package_triplet))
        });

        let value = (
            default_triplet_id(),
            Cow::Borrowed(&self.default),
        );

        std::iter::once(value).chain(other)
    }

    pub fn iter_non_default_triplets(&self) -> impl Iterator<Item = (&TripletId, PackageTriplet)> {
        self.specific_triplets.keys().map(|k| {
            let package_triplet = self.get_triplet_settings(k).unwrap();
            (k, package_triplet)
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

    /// Output binaries for this triplet
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Output binaries for this triplet.")]
    pub out_binaries: Option<Vec<PathBuf>>,
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
