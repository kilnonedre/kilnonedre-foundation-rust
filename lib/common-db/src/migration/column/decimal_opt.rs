use sea_orm::sea_query::{ColumnDef, IntoIden};

pub fn decimal_opt<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column.decimal_len(18, 6);

    column
}
