use sea_orm::sea_query::{ColumnDef, IntoIden};

pub fn created_by<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column.uuid().not_null().comment("创建人 ID");

    column
}

pub fn created_by_aud<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column.uuid().not_null().comment("创建人 ID");

    column
}
