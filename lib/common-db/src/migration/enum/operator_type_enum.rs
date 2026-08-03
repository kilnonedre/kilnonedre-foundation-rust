use sea_orm::sea_query::{Alias, SimpleExpr};

use crate::r#enum::db_enum::{db_enum, db_enum_default};

pub fn db_operator_type_enum() -> Alias {
    db_enum("operator_type")
}

pub fn db_operator_type_default_admin() -> SimpleExpr {
    db_enum_default("operator_type", "ADMIN")
}
