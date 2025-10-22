use serde::{Deserialize, Deserializer, Serializer};
use std::collections::HashMap;
use std::str::FromStr;

pub fn serialize_u64_map<S>(map: &HashMap<u64, u64>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    use serde::ser::SerializeMap;
    let mut map_serializer = serializer.serialize_map(Some(map.len()))?;
    for (k, v) in map {
        map_serializer.serialize_entry(&k.to_string(), v)?;
    }
    map_serializer.end()
}

pub fn deserialize_u64_map<'de, D>(deserializer: D) -> Result<HashMap<u64, u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let str_map = HashMap::<String, u64>::deserialize(deserializer)?;
    str_map
        .into_iter()
        .map(|(k, v)| {
            u64::from_str(&k)
                .map(|k| (k, v))
                .map_err(serde::de::Error::custom)
        })
        .collect()
}
