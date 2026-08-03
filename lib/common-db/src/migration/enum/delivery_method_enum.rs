use sea_orm::sea_query::Alias;

use crate::r#enum::db_enum::db_enum;

pub fn db_delivery_method_enum() -> Alias {
    db_enum("delivery_method")
}
