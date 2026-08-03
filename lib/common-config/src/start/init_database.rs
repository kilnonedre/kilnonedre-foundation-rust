use std::time::Duration;

use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection, Statement};

use crate::env::{DB_HOST, DB_NAME, DB_PASSWORD, DB_PORT, DB_SCHEMA, DB_USER};

pub async fn connect_with_search_path(
    db_url: &str,
    search_path: Option<&str>,
) -> DatabaseConnection {
    let mut opt = ConnectOptions::new(db_url.to_owned());
    if let Some(sp) = search_path {
        opt.set_schema_search_path(sp.to_string()); // 只有在传入时才设置
    }
    opt.max_connections(10)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(8))
        .sqlx_logging(false);
    Database::connect(opt).await.expect("connect db")
}

pub async fn init_db_with_schema() -> DatabaseConnection {
    let db_url = format!(
        "postgres://{}:{}@{}:{}/{}",
        *DB_USER, *DB_PASSWORD, *DB_HOST, *DB_PORT, *DB_NAME
    );

    let bootstrap = connect_with_search_path(&db_url, None).await;

    let be = bootstrap.get_database_backend();
    bootstrap
        .execute(Statement::from_string(
            be,
            format!("CREATE SCHEMA IF NOT EXISTS {}", *DB_SCHEMA),
        ))
        .await
        .expect("create schema");

    drop(bootstrap);

    connect_with_search_path(&db_url, Some(&format!("{},public", *DB_SCHEMA))).await
}

// pub async fn init_public_db() -> DatabaseConnection {
//     let db_url = format!(
//         "postgres://{}:{}@{}:{}/{}",
//         *DB_USER, *DB_PASSWORD, *DB_HOST, *DB_PORT, *DB_NAME
//     );

//     connect_with_search_path(&db_url, None).await
// }
