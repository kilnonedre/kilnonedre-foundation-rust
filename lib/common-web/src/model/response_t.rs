use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct ResponseT<T> {
    pub code: String,
    pub data: Option<T>,
    pub msg: String,
}

#[derive(Debug, Serialize)]
pub struct ListResp<T> {
    pub items: Vec<T>,
}

impl<T> ResponseT<T> {
    pub fn new(code: impl Into<String>, data: Option<T>, msg: impl Into<Option<String>>) -> Self {
        Self {
            code: code.into(),
            data,
            msg: msg.into().unwrap_or_default(),
        }
    }

    pub fn ok(data: T) -> Self {
        Self::new("200", Some(data), Some("ok".to_string()))
    }

    pub fn err(svc: u16, err_no: u16, msg: impl Into<Option<String>>) -> Self {
        Self::new(format!("{svc:03}{err_no:03}"), None, msg)
    }

    pub fn bad_request_err(code: String, msg: impl Into<Option<String>>) -> Self {
        Self::new(code, None, msg)
    }
}

impl<T> ResponseT<ListResp<T>> {
    pub fn ok_list(items: Vec<T>) -> Self {
        Self::new("200", Some(ListResp { items }), Some("ok".to_string()))
    }
}
