use sea_orm::entity::prelude::Json;
use serde::{de::DeserializeOwned, Serialize};

#[track_caller]
pub fn to_json<T: Serialize>(value: &T) -> Json {
    serde_json::to_value(value).expect("JSON 序列化失败")
}

#[track_caller]
pub fn to_json_opt<T: Serialize>(value: &Option<T>) -> Option<Json> {
    value
        .as_ref()
        .map(|v| serde_json::to_value(v).expect("JSON 序列化失败"))
}

#[track_caller]
pub fn from_json<T: DeserializeOwned>(json: &Json) -> T {
    serde_json::from_value(json.clone()).expect("JSON 反序列化失败")
}

#[track_caller]
pub fn from_json_opt<T: DeserializeOwned>(json: &Option<Json>) -> Option<T> {
    json.as_ref()
        .map(|v| serde_json::from_value(v.clone()).expect("JSON 反序列化失败"))
}
