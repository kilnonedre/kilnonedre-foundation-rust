use kilnonedre_common_type::{OperatorContext, OperatorType};
use once_cell::sync::Lazy;

use crate::env::SYS_ID;

pub static SYSTEM_OPERATOR_CONTEXT: Lazy<OperatorContext> = Lazy::new(|| OperatorContext {
    operator_type: OperatorType::Admin,
    user_id: *SYS_ID,
    merchant_id: *SYS_ID,
});
