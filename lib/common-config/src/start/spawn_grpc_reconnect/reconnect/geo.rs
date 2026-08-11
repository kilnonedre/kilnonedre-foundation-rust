use kilnonedre_common_grpc::GrpcGeoLocationClient;

use crate::{
    env::{grpc_host::GRPC_GEO_HOST, grpc_port::GRPC_GEO_PORT},
    start::spawn_grpc_reconnect::spawn_grpc_reconnect,
};

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
