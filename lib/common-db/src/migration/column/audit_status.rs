use sea_orm::sea_query::{ColumnDef, IntoIden};

use crate::r#enum::audit_status_enum::db_audit_status_enum;

pub fn audit_status<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column
        .custom(db_audit_status_enum())
        .not_null()
        .comment("审计状态");

    column
}
