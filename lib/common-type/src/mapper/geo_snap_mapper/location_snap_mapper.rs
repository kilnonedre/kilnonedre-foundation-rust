use crate::{GeoAggregateLocationModel, GeoCompositeLocationModel, GeoLocationSnap};

pub fn composite_to_snap(model: &GeoCompositeLocationModel) -> GeoLocationSnap {
    GeoLocationSnap {
        id: model.id,
        province: model.base.province.clone(),
        city: model.base.city.clone(),
        district: model.base.district.clone(),
        ad_code: model.base.ad_code.clone(),
        address: model.base.address.clone(),
        latitude: model.base.latitude,
        longitude: model.base.longitude,
        poi_id: model.base.poi_id.clone(),
        poi_name: model.base.poi_name.clone(),
        map_provider: model.base.map_provider,
    }
}

pub fn aggregate_to_snap(model: &GeoAggregateLocationModel) -> GeoLocationSnap {
    GeoLocationSnap {
        id: model.id,
        province: model.base.province.clone(),
        city: model.base.city.clone(),
        district: model.base.district.clone(),
        ad_code: model.base.ad_code.clone(),
        address: model.base.address.clone(),
        latitude: model.base.latitude,
        longitude: model.base.longitude,
        poi_id: model.base.poi_id.clone(),
        poi_name: model.base.poi_name.clone(),
        map_provider: model.base.map_provider,
    }
}
