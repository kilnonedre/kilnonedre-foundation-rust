use actix_web::{
    body::{BoxBody, EitherBody},
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    http::header::{self, HeaderName},
    Error, HttpMessage, HttpResponse,
};
use futures_util::future::{ok, LocalBoxFuture, Ready};
use futures_util::FutureExt;
use uuid::Uuid;

use crate::hook::token::jwt::{verify_access_token, Claims};

#[derive(Clone)]
pub struct JwtAuth {
    pub secret: String,

    /// 完全公开：不校验 token，也不解析 token
    pub public_prefixes: Vec<&'static str>,

    /// 可匿名访问：没 token 放行；有 token 就解析，解析失败返回 401
    pub optional_auth_prefixes: Vec<&'static str>,
}

impl JwtAuth {
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: secret.into(),

            public_prefixes: vec![
                "/login",
                "/health",
                "/api/v1/login",
                "/api/v1/we-chat/login",
                "/api/v1/access-token/refresh",
                "/api/v1/merchants/code",
                "/api/v1/we-chat-pay/notify",
                "/api/v1/we-chat-refund/notify",
                "/api/v1/objects",
                "/api/v1/ws",
            ],

            optional_auth_prefixes: vec![
                "/api/v1/product-carousels",
                "/api/v1/products/list",
                "/api/v1/products/",
                "/api/v1/categories",
            ],
        }
    }
}

static X_USER_ID: HeaderName = HeaderName::from_static("x-user-id");

pub struct JwtAuthMiddleware<S> {
    service: S,
    secret: String,
    public_prefixes: Vec<&'static str>,
    optional_auth_prefixes: Vec<&'static str>,
}

impl<S, B> Transform<S, ServiceRequest> for JwtAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<BoxBody, B>>;
    type Error = Error;
    type InitError = ();
    type Transform = JwtAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(JwtAuthMiddleware {
            service,
            secret: self.secret.clone(),
            public_prefixes: self.public_prefixes.clone(),
            optional_auth_prefixes: self.optional_auth_prefixes.clone(),
        })
    }
}

impl<S, B> Service<ServiceRequest> for JwtAuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<BoxBody, B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, mut req: ServiceRequest) -> Self::Future {
        let path = req.path().to_string();

        let is_public = self.public_prefixes.iter().any(|p| path.starts_with(p))
            || path.contains("/swagger-ui")
            || path.contains("/api-docs");

        let is_optional_auth = self
            .optional_auth_prefixes
            .iter()
            .any(|p| path.starts_with(p));

        if is_public {
            return self
                .service
                .call(req)
                .map(|res| res.map(|srv_res| srv_res.map_into_right_body()))
                .boxed_local();
        }

        let auth_header = req.headers().get(header::AUTHORIZATION);

        let token = auth_header
            .and_then(|v| v.to_str().ok())
            .and_then(|v| v.strip_prefix("Bearer "))
            .map(str::trim);

        match token {
            Some(token) if !token.is_empty() => match verify_access_token(token, &self.secret) {
                Ok(token_data) => {
                    let claims: Claims = token_data.claims;
                    let user_id: Uuid = claims.sub;

                    req.extensions_mut().insert(user_id);
                    req.extensions_mut().insert(claims.clone());

                    req.headers_mut().insert(
                        X_USER_ID.clone(),
                        header::HeaderValue::from_str(&user_id.to_string()).unwrap(),
                    );
                }

                Err(_) => {
                    let res = HttpResponse::Unauthorized().finish();

                    return async move { Ok(req.into_response(res.map_into_left_body())) }
                        .boxed_local();
                }
            },

            _ => {
                if !is_optional_auth {
                    let res = HttpResponse::Unauthorized().finish();

                    return async move { Ok(req.into_response(res.map_into_left_body())) }
                        .boxed_local();
                }
            }
        }

        self.service
            .call(req)
            .map(|res| res.map(|srv_res| srv_res.map_into_right_body()))
            .boxed_local()
    }
}
