use serde_json::{ Value, to_value };
use serde::Serialize;
use std::collections::HashMap;

pub fn to_hashmap<T: Serialize>(item: &T) -> HashMap<String, Value> {
    let value = to_value(item).expect("Serialization should succeed");
    let map = value.as_object().expect("Should be an object");

    map.into_iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}
