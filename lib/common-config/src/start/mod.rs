use std::future::Future;

use actix_cors::Cors;
use actix_web::{dev::Server, web, App, HttpServer, Scope};
use kilnonedre_internal_clients::hook::token::preprocess::JwtAuth;
use sea_orm::DatabaseConnection;
use sea_orm_migration::MigratorTrait;
use utoipa::openapi::OpenApi;

// use migration_common::Migrator;

use crate::{
    configure,
    env::{ACCESS_SECRET, APP_PORT, GRPC_PORT, LOG_LEVEL},
    init_logger_with_level, load_env,
    start::app_data::{AppData, AppDataMap},
};

pub mod app_data;
mod init_database;
pub mod redis;
pub mod spawn_grpc_reconnect;
mod start_info;

pub async fn init_app<F>(init_grpc_clients: F) -> std::io::Result<()>
where
    F: FnOnce(),
{
    load_env();
    init_logger_with_level(Some(&LOG_LEVEL));
    init_grpc_clients();

    Ok(())
}

pub async fn init_database<M>() -> DatabaseConnection
where
    M: MigratorTrait,
{
    // let db_public = init_public_db().await;
    // Migrator::up(&db_public, None)
    //     .await
    //     .expect("执行公共数据库迁移失败");

    let db = init_database::init_db_with_schema().await;
    M::up(&db, None).await.expect("执行业务数据库迁移失败");

    db
}

pub async fn start_service_with_grpc<F, Fut>(
    db: DatabaseConnection,
    openapi: OpenApi,
    build_api_route: fn() -> Scope,
    start_grpc_service: F,
    app_data_map: AppDataMap,
) -> std::io::Result<()>
where
    F: FnOnce(DatabaseConnection, String) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    start_info::print_startup_info(*APP_PORT, *GRPC_PORT);

    let grpc_server = start_grpc_server(db.clone(), start_grpc_service);
    let http_server = build_http_server(db, openapi, build_api_route, app_data_map)?;

    tokio::select! {
        _ = grpc_server => {
            println!("gRPC server task exited");
        }

        res = http_server => {
            res?;
        }
    }

    Ok(())
}

pub async fn start_service(
    db: DatabaseConnection,
    openapi: OpenApi,
    build_api_route: fn() -> Scope,
    app_data_map: AppDataMap,
) -> std::io::Result<()> {
    start_info::print_startup_info(*APP_PORT, *GRPC_PORT);

    build_http_server(db, openapi, build_api_route, app_data_map)?.await?;

    Ok(())
}

fn start_grpc_server<F, Fut>(
    db: DatabaseConnection,
    start_grpc_service: F,
) -> tokio::task::JoinHandle<()>
where
    F: FnOnce(DatabaseConnection, String) -> Fut + Send + 'static,
    Fut: Future<Output = ()> + Send + 'static,
{
    let grpc_addr = format!("0.0.0.0:{}", *GRPC_PORT);

    tokio::spawn(async move {
        start_grpc_service(db, grpc_addr).await;
    })
}

fn build_http_server(
    db: DatabaseConnection,
    openapi: OpenApi,
    build_api_route: fn() -> Scope,
    app_data_map: AppDataMap,
) -> std::io::Result<Server> {
    let db_data = web::Data::new(db);

    let kafka_producer = app_data_map.get("kafka").and_then(|v| match v {
        AppData::KafkaProducer(producer) => Some(web::Data::new(producer.clone())),
        _ => None,
    });

    let ws_hub = app_data_map.get("ws_hub").and_then(|v| match v {
        AppData::WsHub(hub) => Some(web::Data::new(hub.clone())),
        _ => None,
    });

    let server = HttpServer::new(move || {
        let mut app = App::new()
            .wrap(JwtAuth::new(ACCESS_SECRET.clone()))
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .max_age(3600),
            )
            .app_data(db_data.clone());

        if let Some(kafka_producer) = kafka_producer.clone() {
            app = app.app_data(kafka_producer);
        }

        if let Some(ws_hub) = ws_hub.clone() {
            app = app.app_data(ws_hub);
        }

        app.configure(configure(openapi.clone(), build_api_route()))
    })
    .bind(("0.0.0.0", *APP_PORT))?
    .run();

    Ok(server)
}
