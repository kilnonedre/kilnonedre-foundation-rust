use sea_orm::sea_query::Alias;

use crate::r#enum::db_enum::db_enum;

pub fn db_storage_method_enum() -> Alias {
    db_enum("storage_method")
}
