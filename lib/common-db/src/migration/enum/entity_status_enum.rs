use sea_orm::sea_query::{Alias, SimpleExpr};

use crate::r#enum::db_enum::{db_enum, db_enum_default};

pub fn db_entity_status_enum() -> Alias {
    db_enum("entity_status")
}

pub fn db_entity_status_default_active() -> SimpleExpr {
    db_enum_default("entity_status", "ACTIVE")
}
