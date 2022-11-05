use std::path::PathBuf;

use crate::models::{dependency::SharedDependency, package::PackageMetadata};

pub trait PackageMetadataExtensions {
    fn get_so_name(&self) -> PathBuf;

    fn get_module_id(&self) -> String {
        let fixed_name = self
            .get_so_name()
            .with_extension("")
            .to_str()
            .unwrap()
            .to_string();
        fixed_name[3..fixed_name.len()].to_string()
    }
}

impl PackageMetadataExtensions for PackageMetadata {
    fn get_so_name(&self) -> PathBuf {
        self.additional_data
            .override_so_name
            .clone()
            .unwrap_or(format!(
                "lib{}_{}.{}",
                self.id,
                self.version.to_string().replace('.', "_"),
                if self.additional_data.static_linking.unwrap_or(false) {
                    "a"
                } else {
                    "so"
                },
            ))
            .into()
    }
}

impl PackageMetadataExtensions for SharedDependency {
    fn get_so_name(&self) -> PathBuf {
        self.dependency
            .additional_data
            .override_so_name
            .clone()
            .unwrap_or(format!(
                "lib{}_{}.{}",
                self.dependency.id,
                self.version.to_string().replace('.', "_"),
                if self
                    .dependency
                    .additional_data
                    .static_linking
                    .unwrap_or(false)
                {
                    "a"
                } else {
                    "so"
                },
            ))
            .into()
    }
}
