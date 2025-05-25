use serde::{Deserialize, Deserializer};

/// 
/// This function is used to deserialize a value from JSON, and if the value is `null`, it will return the default value of the type `T`.
/// 
pub fn deserialize_null_default<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    T: Default + Deserialize<'de>,
    D: Deserializer<'de>,
{
    let opt = Option::deserialize(deserializer)?;
    Ok(opt.unwrap_or_default())
}
