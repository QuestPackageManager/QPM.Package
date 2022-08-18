mod dependency;
pub type Dependency = dependency::Dependency;
pub type SharedDependency = dependency::SharedDependency;

mod package;
pub type Package = package::Package;
pub type SharedPackage = package::SharedPackage;
pub type PackageInfo = package::PackageInfo;
pub type AdditionalPackageData = package::AdditionalPackageData;
pub type CompileOptions = package::CompileOptions;

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_package() -> Result<(), ()> {
        Ok(())
    }
}
