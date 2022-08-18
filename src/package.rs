use crate::Dependency;
use crate::SharedDependency;
use semver::Version;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Package {
    pub shared_dir: PathBuf,
    pub dependencies_dir: PathBuf,
    pub info: PackageInfo,
    pub dependencies: Vec<Dependency>,
    pub additional_data: AdditionalPackageData,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct PackageInfo {
    pub name: String,
    pub id: String,
    pub version: Version,
    pub url: Option<String>,
    pub additional_data: AdditionalPackageData,
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
pub struct AdditionalPackageData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_path: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_qmod: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub static_linking: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_release: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub so_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug_so_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_so_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mod_link: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_files: Option<Vec<String>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename(serialize = "private", deserialize = "private")
    )]
    pub is_private: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub compile_options: Option<CompileOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_folder: Option<String>,
}

/// - compileOptions (QPM.Commands.SupportedPropertiesCommand+CompileOptionsProperty): Additional options for compilation and edits to compilation related files. - Supported in: package
/// Type: QPM.Commands.SupportedPropertiesCommand+CompileOptionsProperty
/// - includePaths - OPTIONAL (System.String[]): Additional include paths to add, relative to the extern directory.
/// - systemIncludes - OPTIONAL (System.String[]): Additional system include paths to add, relative to the extern directory.
/// - cppFeatures - OPTIONAL (System.String[]): Additional C++ features to add.
/// - cppFlags - OPTIONAL (System.String[]): Additional C++ flags to add.
/// - cFlags - OPTIONAL (System.String[]): Additional C flags to add.
#[derive(Serialize, Deserialize, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct CompileOptions {
    /// Additional include paths to add, relative to the extern directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_paths: Option<Vec<String>>,

    /// Additional system include paths to add, relative to the extern directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub system_includes: Option<Vec<String>>,

    /// Additional C++ features to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpp_features: Option<Vec<String>>,

    /// Additional C++ flags to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cpp_flags: Option<Vec<String>>,

    /// Additional C flags to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub c_flags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SharedPackage {
    /// The packageconfig that is stored in qpm.json
    pub config: Package,
    /// The dependencies as given by self.config.resolve()
    pub restored_dependencies: Vec<SharedDependency>,
}
