use sea_orm::sea_query::{ColumnDef, IntoIden};

use crate::r#enum::operator_type_enum::db_operator_type_enum;

pub fn updated_by_type<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column.custom(db_operator_type_enum()).comment("更新人类型");

    column
}

pub fn updated_by_type_aud<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column.custom(db_operator_type_enum()).comment("更新人类型");

    column
}
