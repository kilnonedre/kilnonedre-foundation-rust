use sea_orm::sea_query::{Expr, Iden, SimpleExpr};

pub fn extract_year<C>(column: C) -> SimpleExpr
where
    C: Iden + 'static,
{
    Expr::cust_with_expr("EXTRACT(YEAR FROM ?)", Expr::col(column)).into()
}

pub fn extract_month<C>(column: C) -> SimpleExpr
where
    C: Iden + 'static,
{
    Expr::cust_with_expr("EXTRACT(MONTH FROM ?)", Expr::col(column)).into()
}
