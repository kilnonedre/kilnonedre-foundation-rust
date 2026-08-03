use sea_orm::sea_query::{ColumnDef, Expr, IntoIden};

pub fn uuid_primary_key<T: IntoIden>(iden: T) -> ColumnDef {
    let mut column = ColumnDef::new(iden);

    column
        .uuid()
        .not_null()
        .primary_key()
        .default(Expr::cust("gen_random_uuid()"))
        .comment("主键 ID");

    column
}
