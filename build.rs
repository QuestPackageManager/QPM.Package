use semver::Version;

fn main() {
    let version_result = Version::parse(env!("CARGO_PKG_VERSION"));
    assert!(version_result.is_ok());
}
