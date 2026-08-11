use kilnonedre_common_grpc::{
    GrpcCrmAccountClient, GrpcCrmConsumerProfileClient, GrpcCrmIdentityClient,
    GrpcCrmMerchantClient,
};

use crate::{
    env::{grpc_host::GRPC_CRM_HOST, grpc_port::GRPC_CRM_PORT},
    start::spawn_grpc_reconnect::spawn_grpc_reconnect,
};

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
