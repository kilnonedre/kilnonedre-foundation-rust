use sea_orm::sea_query::Alias;

use crate::r#enum::db_enum::db_enum;

pub fn db_procurement_type_enum() -> Alias {
    db_enum("procurement_type")
}
