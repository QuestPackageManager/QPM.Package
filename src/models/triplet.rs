use std::{borrow::Cow, collections::HashMap, fmt::Display, path::PathBuf};

use schemars::JsonSchema;
use semver::VersionReq;
use serde::{Deserialize, Serialize};

use super::version_req::make_version_req_schema;

use crate::models::{extra::PackageTripletCompileOptions, package::DependencyId};

#[derive(
    Serialize, Deserialize, Clone, Debug, JsonSchema, PartialEq, Eq, Hash, PartialOrd, Ord,
)]
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
    TripletId::default()
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

        let found = self.specific_triplets.get(triplet)?.clone();
        let default = self.default.clone();

        let mut dependencies = found.dependencies;
        dependencies.extend(default.dependencies);

        let mut dev_dependencies = found.dev_dependencies;
        dev_dependencies.extend(default.dev_dependencies);

        let mut env = found.env;
        env.extend(default.env);

        let compile_options = found
            .compile_options
            .clone()
            .map(|a| a.merge(default.compile_options.unwrap_or_default()));

        let qmod_include_files = found
            .qmod_include_files
            .into_iter()
            .chain(default.qmod_include_files)
            .collect();
        
        let qmod_include_dirs = found
            .qmod_include_dirs
            .into_iter()
            .chain(default.qmod_include_dirs)
            .collect();

        Some(PackageTriplet {
            dependencies,
            dev_dependencies,
            env,
            compile_options,
            qmod_include_dirs,
            qmod_include_files,

            out_binaries: found.out_binaries.clone().or(default.out_binaries),
            qmod_url: found.qmod_url.clone().or(default.qmod_url),
            qmod_id: found.qmod_id.clone().or(default.qmod_id),
            qmod_template: found.qmod_template.clone().or(default.qmod_template),
            qmod_output: found.qmod_output.clone().or(default.qmod_output),
            ndk: found.ndk.clone().or(default.ndk),
        })
    }

    pub fn get_triplet(&self, triplet: &TripletId) -> Option<&PackageTriplet> {
        if triplet == &default_triplet_id() {
            return Some(&self.default);
        }

        self.specific_triplets.get(triplet)
    }
    pub fn get_triplet_mut(&mut self, triplet: &TripletId) -> Option<&mut PackageTriplet> {
        if triplet == &default_triplet_id() {
            return Some(&mut self.default);
        }

        self.specific_triplets.get_mut(triplet)
    }

    /// Iterates over all triplets, including the default one.
    pub fn iter_triplets(&self) -> impl Iterator<Item = (TripletId, Cow<PackageTriplet>)> {
        let other = self.specific_triplets.keys().map(|k| {
            let package_triplet = self.get_triplet_settings(k).unwrap();

            (k.clone(), Cow::Owned(package_triplet))
        });

        let value = (default_triplet_id(), Cow::Borrowed(&self.default));

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

    /// QMod ID for this triplet
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "QMod ID for this triplet.")]
    pub qmod_id: Option<String>,

    #[serde(default)]
    #[schemars(description = "List of directories to search during qmod creation.")]
    pub qmod_include_dirs: Vec<PathBuf>,

    #[serde(default)]
    #[schemars(description = "List of files to include in the resulting qmod.")]
    pub qmod_include_files: Vec<PathBuf>,

    #[serde(default)]
    #[schemars(description = "Output path for the qmod.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qmod_output: Option<PathBuf>,

    /// QMod template path for this triplet e.g mod.template.json
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "QMod template for this triplet.")]
    pub qmod_template: Option<PathBuf>,

    /// NDK Version Range
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The NDK version range.")]
    #[schemars(schema_with = "make_version_req_schema")]
    pub ndk: Option<VersionReq>,

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

    /// Target triplet. `default` if null
    #[schemars(description = "Target triplet for this dependency.")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triplet: Option<TripletId>,

    /// Whether to include this dependency in the qmod
    #[serde(default)]
    pub qmod_export: bool,

    /// Whether this is required/optional in the qmod
    /// QMod required field for this dependency
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "QMod required field for this dependency.")]
    pub qmod_required: Option<bool>,
}

impl Display for TripletId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Default for TripletId {
    fn default() -> Self {
        TripletId("default".to_owned())
    }
}
