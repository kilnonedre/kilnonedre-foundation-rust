use sea_orm::sea_query::{Alias, SimpleExpr};

use crate::r#enum::db_enum::{db_enum, db_enum_default};

pub fn db_task_status_enum() -> Alias {
    db_enum("task_status")
}

pub fn db_task_status_default_pending() -> SimpleExpr {
    db_enum_default("task_status", "PENDING")
}
