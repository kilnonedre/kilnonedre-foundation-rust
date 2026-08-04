use actix_web::HttpRequest;
use common_web::{util::error::svc_err_bad_request_msg, ApiError};

use crate::{
    r#type::model::we_chat_pay::WeChatPayNotifyReq,
    r#type::model::we_chat_refund::WeChatRefundTransaction,
    util::we_chat_notify_client::decrypt_resource,
};

pub fn parse_we_chat_refund_notify(
    req: &HttpRequest,
    body: &[u8],
) -> Result<WeChatRefundTransaction, ApiError> {
    // 1. 验签
    crate::util::we_chat_notify_client::verify_notify_signature(req, body)?;

    // 2. 解析微信回调外层结构
    let notify_req: WeChatPayNotifyReq = serde_json::from_slice(body)
        .map_err(|_| svc_err_bad_request_msg(1, 1, "微信退款回调解析失败"))?;

    // 3. 解密 resource
    let plain_text = decrypt_resource(
        &notify_req.resource.ciphertext,
        &notify_req.resource.nonce,
        &notify_req.resource.associated_data,
    )?;

    // 4. 解析退款数据
    let refund: WeChatRefundTransaction = serde_json::from_str(&plain_text)
        .map_err(|_| svc_err_bad_request_msg(1, 1, "微信退款数据解析失败"))?;

    Ok(refund)
}
