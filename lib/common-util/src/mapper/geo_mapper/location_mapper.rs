use common_grpc::geo::GrpcGeoLocationServiceLocationResponse;
use common_misc::util::string_to_uuid::svc_parse_uuid;
use common_type::{svc_to_map_provider, GeoLocationModel, GeoLocationResp};
use common_web::ApiError;

pub fn grpc_to_model(
    grpc_model: GrpcGeoLocationServiceLocationResponse,
) -> Result<GeoLocationResp, ApiError> {
    let id = svc_parse_uuid(&grpc_model.id)?;
    let map_provider = svc_to_map_provider(grpc_model.map_provider)?;

    let result = GeoLocationResp {
        id,
        base: GeoLocationModel {
            province: grpc_model.province,
            city: grpc_model.city,
            district: grpc_model.district,
            ad_code: grpc_model.ad_code,
            address: grpc_model.address,
            latitude: grpc_model.latitude,
            longitude: grpc_model.longitude,
            poi_id: grpc_model.poi_id,
            poi_name: grpc_model.poi_name,
            map_provider,
        },
    };

    Ok(result)
}
