use sea_orm::sea_query::{ColumnDef, Expr, IntoIden};

pub fn created_at<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column
        .timestamp_with_time_zone()
        .not_null()
        .default(Expr::current_timestamp())
        .comment("创建时间");

    column
}

pub fn created_at_aud<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column
        .timestamp_with_time_zone()
        .not_null()
        .comment("创建时间");

    column
}
