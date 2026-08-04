use std::collections::{HashMap, HashSet};

use kilnonedre_common_grpc::{
    geo::{GrpcGeoLocationServiceBatchReadRequest, GrpcGeoLocationServiceCreateRequest},
    GrpcGeoLocationClient,
};
use kilnonedre_common_misc::util::{
    operator_context::operator_context_to_grpc, set_to_vec::uuid_set_to_string_vec,
    string_to_uuid::svc_parse_uuid, vec_to_map::try_vec_to_map_by,
};
use kilnonedre_common_type::{GeoLocationReq, GeoLocationResp, OperatorContext};
use uuid::Uuid;

use kilnonedre_common_web::{util::error::svc_err_internal, ApiError};

use crate::mapper::geo_mapper::location_mapper;

pub async fn list_location_as_map(
    id_set: HashSet<Uuid>,
) -> Result<HashMap<Uuid, GeoLocationResp>, ApiError> {
    let ids = uuid_set_to_string_vec(id_set);

    let resp = GrpcGeoLocationClient::batch_read(GrpcGeoLocationServiceBatchReadRequest { ids })
        .await
        .map_err(|e| svc_err_internal(e, "定位数据获取失败"))?;

    let location_map =
        try_vec_to_map_by(resp.locations, location_mapper::grpc_to_model, |location| {
            location.id
        })?;

    Ok(location_map)
}

pub async fn create_location(
    operator_context: &OperatorContext,
    payload: &GeoLocationReq,
) -> Result<Uuid, ApiError> {
    let grpc_operator_context = operator_context_to_grpc(operator_context);

    let create_location_payload = GrpcGeoLocationServiceCreateRequest {
        operator_context: Some(grpc_operator_context),
        province: payload.location.province.clone(),
        city: payload.location.city.clone(),
        district: payload.location.district.clone(),
        ad_code: payload.location.ad_code.clone(),
        address: payload.location.address.clone(),
        latitude: payload.location.latitude,
        longitude: payload.location.longitude,
        poi_id: payload.location.poi_id.clone(),
        poi_name: payload.location.poi_name.clone(),
        map_provider: payload.location.map_provider.into(),
        raw_response: payload.raw_response.to_string(),
    };

    let create_location_res = GrpcGeoLocationClient::create(create_location_payload)
        .await
        .map_err(|e| svc_err_internal(e, "定位创建失败"))?;

    let location_id = svc_parse_uuid(&create_location_res.id)?;

    Ok(location_id)
}

pub async fn create_location_opt(
    operator_context: &OperatorContext,
    location_opt: &Option<GeoLocationReq>,
) -> Result<Option<Uuid>, ApiError> {
    if let Some(location) = location_opt.clone() {
        let location_id = create_location(operator_context, &location).await?;
        Ok(Some(location_id))
    } else {
        Ok(None)
    }
}
