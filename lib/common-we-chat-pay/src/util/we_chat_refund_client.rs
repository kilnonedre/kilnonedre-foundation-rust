use chrono::Utc;
use kilnonedre_common_config::env::we_chat::WE_CHAT_REFUND_NOTIFY_URL;
use kilnonedre_common_web::{util::error::svc_err_internal_msg, ApiError};

use crate::{
    r#type::model::we_chat_refund::{WeChatRefundAmount, WeChatRefundReq, WeChatRefundResp},
    util::we_chat_pay_client::{build_authorization, gen_nonce_str},
};

pub async fn create_we_chat_refund(
    out_trade_no: &str,
    out_refund_no: &str,
    refund_amount_fen: i64,
    total_fen: i64,
    reason: &str,
) -> Result<WeChatRefundResp, ApiError> {
    let path = "/v3/refund/domestic/refunds";

    let url = format!("https://api.mch.weixin.qq.com{}", path);

    let body = WeChatRefundReq {
        out_trade_no: out_trade_no.to_string(),
        out_refund_no: out_refund_no.to_string(),
        reason: reason.to_string(),
        notify_url: WE_CHAT_REFUND_NOTIFY_URL.clone(),
        amount: WeChatRefundAmount {
            refund: refund_amount_fen,
            total: total_fen,
            currency: "CNY".to_string(),
        },
    };

    let body_json =
        serde_json::to_string(&body).map_err(|_| svc_err_internal_msg("微信退款请求序列化失败"))?;

    let timestamp = Utc::now().timestamp().to_string();

    let nonce_str = gen_nonce_str();

    let authorization = build_authorization("POST", path, &timestamp, &nonce_str, &body_json)?;

    let resp = reqwest::Client::new()
        .post(url)
        .header("Authorization", authorization)
        .header("Content-Type", "application/json")
        .header("Accept", "application/json")
        .header("User-Agent", "rust-reqwest")
        .body(body_json)
        .send()
        .await
        .map_err(|_| svc_err_internal_msg("请求微信退款失败"))?;

    let status = resp.status();

    let text = resp
        .text()
        .await
        .map_err(|_| svc_err_internal_msg("读取微信退款响应失败"))?;

    if !status.is_success() {
        return Err(svc_err_internal_msg(&format!(
            "微信退款失败: {} {}",
            status, text
        )));
    }

    let data: WeChatRefundResp =
        serde_json::from_str(&text).map_err(|_| svc_err_internal_msg("解析微信退款响应失败"))?;

    Ok(data)
}
