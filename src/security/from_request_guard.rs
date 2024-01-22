// We are using some guard via fromrequest because a actix guard cannot reach the data of the request, so cannot reach the JWT config
// The fromRequest will be executed before the route controler and can emit the custom issue define here in case of KO.

use actix_web::{web::Data, FromRequest, HttpResponse, ResponseError};
use std::future::{ready, Ready};

use crate::config::config_structs::AppConfig;

use super::jwt::validate_authorization;
use super::jwt_struct::Claims;

// Custom error Unauthorized
#[derive(Debug)]
pub struct NotAuthorized {}

impl std::fmt::Display for NotAuthorized {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Unauthorized")
    }
}

impl ResponseError for NotAuthorized {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::Unauthorized().body("Unauthorized")
    }
}

pub struct JWTGuard {
    pub claims: Claims,
}

impl FromRequest for JWTGuard {
    type Error = NotAuthorized;
    type Future = Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Self::Future {
        // Extract config from request
        let conf: &Data<AppConfig> = req.app_data().expect("Config is not reachable");
        let jwt_secret_key = conf.administration.secret_key.as_slice();
        // Extract JWT token from request Header and validate it, extracting claims.
        match req
            .headers()
            .get("Authorization")
            .ok_or(NotAuthorized {})
            .and_then(|authorization| {
                validate_authorization(
                    jwt_secret_key,
                    authorization.to_str().map_err(|_| NotAuthorized {})?,
                )
                .map_err(|_| NotAuthorized {})
            }) {
            Ok(claims) => ready(Ok(JWTGuard { claims })),
            Err(_) => ready(Err(NotAuthorized {})),
        }
    }
}
