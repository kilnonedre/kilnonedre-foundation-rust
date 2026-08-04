use chrono::Utc;
use kilnonedre_common_type::{OperatorContext, OrderType};
use rand::Rng;
use uuid::Uuid;

fn extract_suffix(id: &Uuid) -> u16 {
    let bytes = id.as_bytes();
    let value = u16::from_be_bytes([bytes[14], bytes[15]]);
    value % 1000
}

fn generate_no(prefix: &OrderType, operator_context: &OperatorContext) -> String {
    let merchant = extract_suffix(&operator_context.merchant_id);
    let operator = extract_suffix(&operator_context.user_id);
    let time = Utc::now().format("%Y%m%d%H%M%S%3f");
    let random = rand::rng().random_range(0..10_000_000);
    format!("{prefix}{time}{merchant:03}{operator:03}{random:07}")
}

pub fn generate_order_no(operator_context: &OperatorContext) -> String {
    generate_no(&OrderType::OD, operator_context)
}

pub fn generate_purchase_order_no(operator_context: &OperatorContext) -> String {
    generate_no(&OrderType::PO, operator_context)
}

pub fn generate_inbound_order_no(operator_context: &OperatorContext) -> String {
    generate_no(&OrderType::IO, operator_context)
}
