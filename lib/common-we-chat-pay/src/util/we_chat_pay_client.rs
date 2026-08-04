use base64::{engine::general_purpose, Engine};
use chrono::Utc;
use common_config::env::we_chat::{
    WE_CHAT_APP_ID, WE_CHAT_PAY_MCH_ID, WE_CHAT_PAY_NOTIFY_URL, WE_CHAT_PAY_PRIVATE_KEY_PATH,
    WE_CHAT_PAY_SERIAL_NO,
};
use common_misc::util::amount::svc_decimal_yuan_to_fen;
use common_web::{util::error::svc_err_internal_msg, ApiError};
use openssl::{hash::MessageDigest, pkey::PKey, sign::Signer};
use rand::{distr::Alphanumeric, Rng};
use rust_decimal::Decimal;

use crate::r#type::{
    model::we_chat_pay::{JsapiAmount, JsapiPayer, JsapiPrepayReq, JsapiPrepayResp},
    response::we_chat_pay_resp::CreateWeChatPayResp,
};

pub async fn create_jsapi_prepay(
    open_id: &str,
    no: &str,
    pay_mount: &Decimal,
) -> Result<String, ApiError> {
    if open_id.trim().is_empty() {
        return Err(svc_err_internal_msg("缺少 open_id"));
    }
    let path = "/v3/pay/transactions/jsapi";
    let url = format!("https://api.mch.weixin.qq.com{}", path);

    let body = JsapiPrepayReq {
        appid: WE_CHAT_APP_ID.clone(),
        mchid: WE_CHAT_PAY_MCH_ID.clone(),
        description: format!("订单支付-{}", no),
        out_trade_no: no.to_string(),
        notify_url: WE_CHAT_PAY_NOTIFY_URL.clone(),
        amount: JsapiAmount {
            total: svc_decimal_yuan_to_fen(*pay_mount)?,
            currency: "CNY".to_string(),
        },
        payer: JsapiPayer {
            openid: open_id.to_string(),
        },
    };
    let body_json = serde_json::to_string(&body).map_err(|_| svc_err_internal_msg("序列化失败"))?;

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
        .map_err(|_| svc_err_internal_msg("请求微信失败"))?;

    let status = resp.status();

    let text = resp
        .text()
        .await
        .map_err(|_| svc_err_internal_msg("读取响应失败"))?;

    if !status.is_success() {
        return Err(svc_err_internal_msg(&format!(
            "微信下单失败: {} {}",
            status, text
        )));
    }

    let data: JsapiPrepayResp =
        serde_json::from_str(&text).map_err(|_| svc_err_internal_msg("解析失败"))?;

    Ok(data.prepay_id)
}

pub fn build_we_chat_pay_resp(prepay_id: &str) -> Result<CreateWeChatPayResp, ApiError> {
    let time_stamp = Utc::now().timestamp().to_string();
    let nonce_str = gen_nonce_str();
    let package = format!("prepay_id={}", prepay_id);
    let message = format!(
        "{}\n{}\n{}\n{}\n",
        *WE_CHAT_APP_ID, time_stamp, nonce_str, package
    );
    let pay_sign = rsa_sign(message.as_bytes())?;
    Ok(CreateWeChatPayResp {
        time_stamp,
        nonce_str,
        package,
        sign_type: "RSA".to_string(),
        pay_sign,
    })
}

pub fn gen_nonce_str() -> String {
    rand::rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

// 构造微信支付 Authorization 头
pub fn build_authorization(
    method: &str,
    canonical_url: &str,
    timestamp: &str,
    nonce_str: &str,
    body: &str,
) -> Result<String, ApiError> {
    let message = format!(
        "{}\n{}\n{}\n{}\n{}\n",
        method, canonical_url, timestamp, nonce_str, body
    );

    let signature = rsa_sign(message.as_bytes())?;

    Ok(format!(
        r#"WECHATPAY2-SHA256-RSA2048 mchid="{}",nonce_str="{}",signature="{}",timestamp="{}",serial_no="{}""#,
        *WE_CHAT_PAY_MCH_ID, nonce_str, signature, timestamp, *WE_CHAT_PAY_SERIAL_NO
    ))
}

// RSA-SHA256 签名
fn rsa_sign(message: &[u8]) -> Result<String, ApiError> {
    let private_key_pem = std::fs::read_to_string(&*WE_CHAT_PAY_PRIVATE_KEY_PATH)
        .map_err(|_| svc_err_internal_msg("私钥文件读取失败"))?;

    let key = PKey::private_key_from_pem(private_key_pem.as_bytes())
        .map_err(|_| svc_err_internal_msg("私钥解析失败"))?;

    let mut signer = Signer::new(MessageDigest::sha256(), &key)
        .map_err(|_| svc_err_internal_msg("签名器创建失败"))?;

    signer
        .update(message)
        .map_err(|_| svc_err_internal_msg("签名写入失败"))?;

    let sign = signer
        .sign_to_vec()
        .map_err(|_| svc_err_internal_msg("签名失败"))?;

    Ok(general_purpose::STANDARD.encode(sign))
}
