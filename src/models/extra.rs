use std::path::PathBuf;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Hash, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Additional metadata for the package.")]
pub struct AdditionalPackageMetadata {
    /// Whether or not the package is header only
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Whether or not the package is header only")]
    pub headers_only: Option<bool>,

    /// Whether or not the package is statically linked
    /// DEPRECATED
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(since="0.2.0", note="Use static_link instead")]
    #[schemars(description = "Whether the package is statically linked. Deprecated, use staticLink instead.")]
    pub static_linking: Option<bool>,

    /// the link to the so file
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The link to the shared object file.")]
    pub so_link: Option<String>,

    /// the link to the so file
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The link to the static library file.")]
    pub static_link: Option<String>,

    /// the link to the debug .so file
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The link to the debug shared object file.")]
    pub debug_so_link: Option<String>,

    /// the overridden so file name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The override name for the shared object file.")]
    pub override_so_name: Option<String>,

    /// the overridden static file name
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The override name for the static library file.")]
    pub override_static_name: Option<String>,

    /// the link to the qmod
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The link to the qmod file.")]
    pub mod_link: Option<String>,

    /// Branch name of a Github repo. Only used when a valid github url is provided
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "The branch name of a GitHub repository. Only used when a valid GitHub URL is provided.")]
    pub branch_name: Option<String>,

    /// Additional Compile options to be used with this package
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Additional compile options for the package.")]
    pub compile_options: Option<CompileOptions>,

    /// Sub folder to use from the downloaded repo / zip, so one repo can contain multiple packages
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Sub-folder to use from the downloaded repository or zip, so one repository can contain multiple packages.")]
    pub sub_folder: Option<String>,

    /// Whether to generate the cmake files on restore
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Whether to generate CMake files on restore.")]
    pub cmake: Option<bool>,

    /// Whether to generate the a toolchain JSON file [CompileOptions] describing the project setup configuration
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Path to generate a toolchain JSON file describing the project setup configuration.")]
    pub toolchain_out: Option<PathBuf>
}

/// - compileOptions (QPM.Commands.SupportedPropertiesCommand+CompileOptionsProperty): Additional options for compilation and edits to compilation related files. - Supported in: package
/// Type: QPM.Commands.SupportedPropertiesCommand+CompileOptionsProperty
/// - includePaths - OPTIONAL (System.String[]): Additional include paths to add, relative to the extern directory.
/// - systemIncludes - OPTIONAL (System.String[]): Additional system include paths to add, relative to the extern directory.
/// - cppFeatures - OPTIONAL (System.String[]): Additional C++ features to add.
/// - cppFlags - OPTIONAL (System.String[]): Additional C++ flags to add.
/// - cFlags - OPTIONAL (System.String[]): Additional C flags to add.
#[derive(Serialize, Deserialize, JsonSchema, Default, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Additional options for compilation and edits to compilation related files.")]
pub struct CompileOptions {
    /// Additional include paths to add, relative to the extern directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Additional include paths to add, relative to the extern directory.")]
    pub include_paths: Option<Vec<String>>,

    /// Additional system include paths to add, relative to the extern directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Additional system include paths to add, relative to the extern directory.")]
    pub system_includes: Option<Vec<String>>,

    /// Additional C++ features to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[deprecated(since="0.2.1", note="Unused and exclusive to CMake")]
    #[schemars(description = "Additional C++ features to add. Deprecated, unused and exclusive to CMake.")]
    pub cpp_features: Option<Vec<String>>,

    /// Additional C++ flags to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Additional C++ flags to add.")]
    pub cpp_flags: Option<Vec<String>>,

    /// Additional C flags to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Additional C flags to add.")]
    pub c_flags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Describes the dependency type.")]
pub enum DependencyLibType {
    #[schemars(description = "Shared library")]
    Shared, // shared

    #[schemars(description = "Static library")]
    Static, // statically link

    #[schemars(description = "Header only")]
    HeaderOnly // Only restore headers, don't link
}

// Modifies how a package should be restored in qpm.json
#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug, Hash, Eq, PartialEq, Default)]
#[serde(rename_all = "camelCase")]
#[schemars(description = "Modifies how a dependency should be restored.")]
pub struct PackageDependencyModifier {
    /// Copy a dependency from a location that is local to this root path instead of from a remote url
    /// Technically just a dependency field
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Copy a dependency from a location that is local to this root path instead of from a remote URL.")]
    pub local_path: Option<String>,

    /// By default if empty, true
    /// If false, this mod dependency will NOT be included in the generated mod.json
    /// Technically just a dependency field
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "If the mod dependency should be included in the generated mod.json. Defaults to true.")]
    pub include_qmod: Option<bool>,

    /// Specify any additional files to be downloaded
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Additional files to be downloaded.")]
    pub extra_files: Option<Vec<String>>,

    /// Whether or not the dependency is private and should be used in restore
    /// Technically just a dependency field
    #[serde(
        skip_serializing_if = "Option::is_none",
        rename(serialize = "private", deserialize = "private")
    )]
    #[schemars(description = "Whether or not the dependency is private and should be used in restore.")]
    pub is_private: Option<bool>,

    /// Specifies how to restore this dependency
    #[serde(
        skip_serializing_if = "Option::is_none",
    )]
    #[schemars(description = "Specifies how to restore this dependency.")]
    pub lib_type: Option<DependencyLibType>,

    /// whether the mod is optional or required. If omitted, assume Some(True)
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "required")]
    #[schemars(description = "Whether the mod is optional or required. If omitted, assume true.")]
    pub required: Option<bool>,
}
