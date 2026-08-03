use sea_orm::sea_query::Alias;

use crate::r#enum::db_enum::db_enum;

pub fn db_warehouse_bin_type_enum() -> Alias {
    db_enum("warehouse_bin_type")
}
