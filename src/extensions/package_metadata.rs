use std::path::PathBuf;

use crate::models::{package::Package, shared_package::SharedPackage};

pub trait PackageMetadataExtensions {}

impl PackageMetadataExtensions for Package {}

