use common_web::ApiError;
use uuid::Uuid;

use crate::util::string_to_uuid::svc_parse_uuid;

pub fn uuid_opt_to_string_opt(value: Option<Uuid>) -> Option<String> {
    value.map(|id| id.to_string())
}

pub fn uuid_vec_opt_to_string_vec(value: Option<Vec<Uuid>>) -> Vec<String> {
    value
        .unwrap_or_default()
        .into_iter()
        .map(|id| id.to_string())
        .collect()
}

pub fn uuid_vec_to_string_vec(value: Vec<Uuid>) -> Vec<String> {
    value.into_iter().map(|id| id.to_string()).collect()
}

pub fn string_vec_to_uuid_vec(value: Vec<String>) -> Result<Vec<Uuid>, ApiError> {
    value.iter().map(|id| svc_parse_uuid(id)).collect()
}

pub fn string_vec_opt_to_uuid_vec_opt(
    value: Option<Vec<String>>,
) -> Result<Option<Vec<Uuid>>, ApiError> {
    value.map(string_vec_to_uuid_vec).transpose()
}
