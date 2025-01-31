mod models;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};


fn main() {
    let schema = schemars::schema_for!(models::package::PackageConfig);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
