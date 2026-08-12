use sea_orm::sea_query::{Expr, SimpleExpr};

pub fn extract_year<E>(expr: E) -> SimpleExpr
where
    E: Into<SimpleExpr>,
{
    Expr::cust_with_expr("EXTRACT(YEAR FROM ?)", expr).into()
}

pub fn extract_month<E>(expr: E) -> SimpleExpr
where
    E: Into<SimpleExpr>,
{
    Expr::cust_with_expr("EXTRACT(MONTH FROM ?)", expr).into()
}
