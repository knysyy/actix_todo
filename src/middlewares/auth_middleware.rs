use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{
    future::{ok, Ready},
    Future,
};

use actix_web::{
    dev::{Service, ServiceRequest, ServiceResponse, Transform},
    http::{header, HeaderValue, Method, StatusCode},
    web::Data,
    Error,
};

use crate::models::user_token::UserToken;
use crate::{
    config::{constants, database::Pool},
    models::{response::ErrorResponse, user::User},
};

pub struct Authentication;

impl<S, B> Transform<S> for Authentication
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthenticationMiddleware { service })
    }
}

pub struct AuthenticationMiddleware<S> {
    service: S,
}

impl<S, B> Service for AuthenticationMiddleware<S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, mut req: ServiceRequest) -> Self::Future {
        let mut authentication_pass: bool = false;

        let headers = req.headers_mut();
        headers.append(header::CONTENT_LENGTH, HeaderValue::from_static("true"));
        if Method::OPTIONS == *req.method() {
            authentication_pass = true;
        } else {
            for ignore_route in constants::IGNORE_ROUTES.iter() {
                if req.path().starts_with(ignore_route) {
                    authentication_pass = true;
                    break;
                }
            }
        }
        if !authentication_pass {
            if let Some(pool) = req.app_data::<Data<Pool>>() {
                if let Some(auth_header) = req.headers().get(constants::AUTHORIZATION) {
                    if let Ok(auth_str) = auth_header.to_str() {
                        if auth_str.starts_with("bearer") || auth_str.starts_with("Bearer") {
                            let token = auth_str[6..auth_str.len()].trim();
                            if let Ok(token_data) = UserToken::decode_token(token.to_string()) {
                                if User::is_valid_login_session(
                                    &token_data.claims,
                                    &pool.get().unwrap(),
                                ) {
                                    authentication_pass = true;
                                } else {
                                    error!("Invalid token");
                                }
                            }
                        }
                    }
                }
            }
        }
        if authentication_pass {
            let fut = self.service.call(req);
            Box::pin(async move {
                let res = fut.await?;
                Ok(res)
            })
        } else {
            Box::pin(async move {
                Ok(req.into_response(
                    ErrorResponse::new(StatusCode::UNAUTHORIZED, "invalid Token")
                        .build()
                        .into_body(),
                ))
            })
        }
    }
}
