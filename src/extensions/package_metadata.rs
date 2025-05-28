use std::path::PathBuf;

use crate::models::{package::PackageConfig, shared_package::SharedPackageConfig};

pub trait PackageMetadataExtensions {}

impl PackageMetadataExtensions for PackageConfig {}
