use sea_orm::sea_query::Alias;

use crate::r#enum::db_enum::db_enum;

pub fn db_purchase_order_status_enum() -> Alias {
    db_enum("purchase_order_status")
}
