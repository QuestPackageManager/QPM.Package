mod models;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let schema = if args.contains(&"--shared".to_string()) {
        schemars::schema_for!(models::dependency::SharedPackageConfig)
    } else {
        schemars::schema_for!(models::package::PackageConfig)
    };

    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}
