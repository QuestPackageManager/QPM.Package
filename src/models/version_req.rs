use schemars::{JsonSchema, r#gen::SchemaGenerator, schema::Schema, schema_for};

pub fn make_version_req_schema(generator: &mut SchemaGenerator) -> Schema {
    let schema = String::json_schema(generator);
    let mut schema = schema.into_object();

    schema
        .object()
        .properties
        .insert("format".into(), schema_for!(String).schema.into());
    Schema::Object(schema)
}
