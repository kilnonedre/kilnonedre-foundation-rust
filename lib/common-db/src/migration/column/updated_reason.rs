use sea_orm::sea_query::{ColumnDef, IntoIden};

pub fn updated_reason<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column.text().comment("更新原因");

    column
}

pub fn updated_reason_aud<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column.text().comment("更新原因");

    column
}
