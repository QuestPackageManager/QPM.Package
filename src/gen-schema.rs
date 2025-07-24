use models::{package::PackageConfig, shared_package::SharedPackageConfig};

pub mod extensions;
pub mod models;

fn main() {
    let shared_schema_json = schemars::schema_for!(SharedPackageConfig);
    let shared_schema = serde_json::to_string_pretty(&shared_schema_json).unwrap();
    std::fs::write("qpm.shared.schema.json", shared_schema).expect("Failed to write shared schema");

    let schema_json = schemars::schema_for!(PackageConfig);
    let schema = serde_json::to_string_pretty(&schema_json).unwrap();
    std::fs::write("qpm.schema.json", schema).expect("Failed to write schema");
}
