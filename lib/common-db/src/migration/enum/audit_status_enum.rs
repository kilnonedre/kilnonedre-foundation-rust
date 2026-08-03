use sea_orm::sea_query::{Alias, SimpleExpr};

use crate::r#enum::db_enum::{db_enum, db_enum_default};

pub fn db_audit_status_enum() -> Alias {
    db_enum("audit_status")
}

pub fn db_audit_status_default_create() -> SimpleExpr {
    db_enum_default("audit_status", "CREATE")
}
