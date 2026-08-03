use sea_orm::sea_query::{ColumnDef, IntoIden};

use crate::r#enum::operator_type_enum::db_operator_type_enum;

pub fn created_by_type<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column
        .custom(db_operator_type_enum())
        .not_null()
        .comment("创建人类型");

    column
}

pub fn created_by_type_aud<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column
        .custom(db_operator_type_enum())
        .not_null()
        .comment("创建人类型");

    column
}
