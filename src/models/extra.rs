use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// - compileOptions (QPM.Commands.SupportedPropertiesCommand+CompileOptionsProperty): Additional options for compilation and edits to compilation related files. - Supported in: package
///   Type: QPM.Commands.SupportedPropertiesCommand+CompileOptionsProperty
/// - includePaths - OPTIONAL (System.String[]): Additional include paths to add, relative to the extern directory.
/// - systemIncludes - OPTIONAL (System.String[]): Additional system include paths to add, relative to the extern directory.
/// - cppFeatures - OPTIONAL (System.String[]): Additional C++ features to add.
/// - cppFlags - OPTIONAL (System.String[]): Additional C++ flags to add.
/// - cFlags - OPTIONAL (System.String[]): Additional C flags to add.
#[derive(Serialize, Deserialize, JsonSchema, Default, Clone, Debug, Hash, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
#[schemars(
    description = "Additional options for compilation and edits to compilation related files."
)]
pub struct PackageCompileOptions {
    /// Additional include paths to add, relative to the extern package's shared directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Additional include paths to add, relative to the extern package's shared directory.")]
    pub include_paths: Option<Vec<String>>,

    /// Additional system include paths to add, relative to the extern package's shared directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(
        description = "Additional system include paths to add, relative to the extern package's shared directory."
    )]
    pub system_includes: Option<Vec<String>>,

    /// Additional C++ flags to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Additional C++ flags to add.")]
    pub cpp_flags: Option<Vec<String>>,

    /// Additional C flags to add.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schemars(description = "Additional C flags to add.")]
    pub c_flags: Option<Vec<String>>,
}

impl PackageCompileOptions {
    pub fn merge(self, other: PackageCompileOptions) -> Self {
        Self {
            c_flags: self.c_flags.or(other.c_flags),
            cpp_flags: self.cpp_flags.or(other.cpp_flags),
            include_paths: self.include_paths.or(other.include_paths),
            system_includes: self.system_includes.or(other.system_includes),
        }
    }
}
