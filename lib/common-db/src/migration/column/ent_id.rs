use sea_orm::sea_query::{ColumnDef, IntoIden};

pub fn ent_id<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column.uuid().not_null().comment("原实体 ID");

    column
}
