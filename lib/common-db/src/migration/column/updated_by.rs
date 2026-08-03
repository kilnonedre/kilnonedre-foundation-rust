use sea_orm::sea_query::{ColumnDef, IntoIden};

pub fn updated_by<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column.uuid().comment("更新人 ID");

    column
}

pub fn updated_by_aud<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column.uuid().comment("更新人 ID");

    column
}
