use sea_orm::sea_query::Alias;

use crate::r#enum::db_enum::db_enum;

pub fn db_map_provider_enum() -> Alias {
    db_enum("map_provider")
}
