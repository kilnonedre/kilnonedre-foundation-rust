use actix_web::{web::ServiceConfig, Scope};
use utoipa::openapi::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::env::SVC_NAME;

pub fn configure(openapi: OpenApi, route: Scope) -> impl FnOnce(&mut ServiceConfig) {
    let prefixed_openapi_url = format!("/api/{}/api-docs/openapi.json", *SVC_NAME);

    move |cfg: &mut ServiceConfig| {
        let swagger = if cfg!(debug_assertions) {
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url("/api-docs/openapi.json", openapi.clone())
                .url(prefixed_openapi_url, openapi.clone())
        } else {
            SwaggerUi::new("/swagger-ui/{_:.*}")
                .url(prefixed_openapi_url, openapi.clone())
                .url("/api-docs/openapi.json", openapi.clone())
        };

        cfg.service(route).service(swagger);
    }
}
