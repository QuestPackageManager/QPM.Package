pub mod models;
pub mod extensions;

use models::{package::Package, shared_package::SharedPackage};


fn main() {
    let shared_schema_json = schemars::schema_for!(SharedPackage);
    let shared_schema = serde_json::to_string_pretty(&shared_schema_json).unwrap();
    std::fs::write("qpm.shared.schema.json", shared_schema).expect("Failed to write shared schema");

    let schema_json = schemars::schema_for!(Package);
    let schema = serde_json::to_string_pretty(&schema_json).unwrap();
    std::fs::write("qpm.schema.json", schema).expect("Failed to write schema");
}
