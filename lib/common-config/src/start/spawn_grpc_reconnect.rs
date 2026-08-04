use kilnonedre_common_grpc::{
    GrpcCrmAccountClient, GrpcCrmConsumerProfileClient, GrpcCrmIdentityClient,
    GrpcCrmMerchantClient, GrpcGeoLocationClient, GrpcLogisticsRouteClient,
    GrpcProcurementPurchaserClient, GrpcProcurementSupplierClient, GrpcWmsWarehouseClient,
    WorkflowProcessGrpcClient,
};
use std::future::Future;
use std::time::Duration;
use tokio::time::sleep;

use crate::env::grpc_host::{
    GRPC_CRM_HOST, GRPC_GEO_HOST, GRPC_LOGISTIC_HOST, GRPC_PROCUREMENT_HOST, GRPC_WMS_HOST,
    GRPC_WORKFLOW_HOST,
};
use crate::env::grpc_port::{
    GRPC_CRM_PORT, GRPC_GEO_PORT, GRPC_LOGISTIC_PORT, GRPC_PROCUREMENT_PORT, GRPC_WMS_PORT,
    GRPC_WORKFLOW_PORT,
};

fn spawn_grpc_reconnect<F, Fut>(name: &'static str, addr: String, init: F)
where
    F: Fn(String) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync>>> + Send + 'static,
{
    tokio::spawn(async move {
        let mut backoff = Duration::from_secs(1);
        let max_backoff = Duration::from_secs(30);

        loop {
            log::debug!("🔄 {} 尝试连接: {}", name, addr);

            match init(addr.clone()).await {
                Ok(_) => {
                    log::info!("✅ {} gRPC 连接就绪: {}", name, addr);
                    sleep(Duration::from_secs(300)).await;
                    backoff = Duration::from_secs(1);
                }
                Err(e) => {
                    log::warn!(
                        "⚠️ {} gRPC 连接失败: {}, 将在 {:?} 后重试，err={}",
                        name,
                        addr,
                        backoff,
                        e
                    );
                    sleep(backoff).await;
                    backoff = (backoff * 2).min(max_backoff);
                }
            }
        }
    });
}

pub fn spawn_crm_account_grpc_reconnect() {
    spawn_grpc_reconnect(
        "CrmAccount",
        format!("{}:{}", *GRPC_CRM_HOST, *GRPC_CRM_PORT),
        |addr| async move {
            GrpcCrmAccountClient::init(&addr)
                .await
                .map_err(|e| e.into())
        },
    );
}

pub fn spawn_crm_identity_grpc_reconnect() {
    spawn_grpc_reconnect(
        "CrmIdentity",
        format!("{}:{}", *GRPC_CRM_HOST, *GRPC_CRM_PORT),
        |addr| async move {
            GrpcCrmIdentityClient::init(&addr)
                .await
                .map_err(|e| e.into())
        },
    );
}

pub fn spawn_crm_consumer_profile_grpc_reconnect() {
    spawn_grpc_reconnect(
        "CrmConsumerProfile",
        format!("{}:{}", *GRPC_CRM_HOST, *GRPC_CRM_PORT),
        |addr| async move {
            GrpcCrmConsumerProfileClient::init(&addr)
                .await
                .map_err(|e| e.into())
        },
    );
}

pub fn spawn_crm_merchant_grpc_reconnect() {
    spawn_grpc_reconnect(
        "CrmMerchant",
        format!("{}:{}", *GRPC_CRM_HOST, *GRPC_CRM_PORT),
        |addr| async move {
            GrpcCrmMerchantClient::init(&addr)
                .await
                .map_err(|e| e.into())
        },
    );
}

pub fn spawn_workflow_process_grpc_reconnect() {
    spawn_grpc_reconnect(
        "WorkflowProcess",
        format!("{}:{}", *GRPC_WORKFLOW_HOST, *GRPC_WORKFLOW_PORT),
        |addr| async move {
            WorkflowProcessGrpcClient::init(&addr)
                .await
                .map_err(|e| e.into())
        },
    );
}

pub fn spawn_geo_location_grpc_reconnect() {
    spawn_grpc_reconnect(
        "GeoLocation",
        format!("{}:{}", *GRPC_GEO_HOST, *GRPC_GEO_PORT),
        |addr| async move {
            GrpcGeoLocationClient::init(&addr)
                .await
                .map_err(|e| e.into())
        },
    );
}

pub fn spawn_logistics_route_grpc_reconnect() {
    spawn_grpc_reconnect(
        "LogisticsRoute",
        format!("{}:{}", *GRPC_LOGISTIC_HOST, *GRPC_LOGISTIC_PORT),
        |addr| async move {
            GrpcLogisticsRouteClient::init(&addr)
                .await
                .map_err(|e| e.into())
        },
    );
}

pub fn spawn_procurement_purchaser_grpc_reconnect() {
    spawn_grpc_reconnect(
        "ProcurementPurchaser",
        format!("{}:{}", *GRPC_PROCUREMENT_HOST, *GRPC_PROCUREMENT_PORT),
        |addr| async move {
            GrpcProcurementPurchaserClient::init(&addr)
                .await
                .map_err(|e| e.into())
        },
    );
}

pub fn spawn_procurement_supplier_grpc_reconnect() {
    spawn_grpc_reconnect(
        "ProcurementSupplier",
        format!("{}:{}", *GRPC_PROCUREMENT_HOST, *GRPC_PROCUREMENT_PORT),
        |addr| async move {
            GrpcProcurementSupplierClient::init(&addr)
                .await
                .map_err(|e| e.into())
        },
    );
}

pub fn spawn_wms_warehouse_grpc_reconnect() {
    spawn_grpc_reconnect(
        "WmsWarehouse",
        format!("{}:{}", *GRPC_WMS_HOST, *GRPC_WMS_PORT),
        |addr| async move {
            GrpcWmsWarehouseClient::init(&addr)
                .await
                .map_err(|e| e.into())
        },
    );
}
