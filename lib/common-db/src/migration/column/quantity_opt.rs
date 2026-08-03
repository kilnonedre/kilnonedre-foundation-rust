use sea_orm::sea_query::{ColumnDef, IntoIden};

/// 数量（18,6）
pub fn quantity_opt<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column.decimal_len(18, 6);

    column
}
